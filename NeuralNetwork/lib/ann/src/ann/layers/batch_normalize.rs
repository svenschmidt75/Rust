use crate::ann::layers::layer::Layer;
use crate::ann::minibatch::Minibatch;
use linear_algebra::ops;
use linear_algebra::vector::Vector;


const EPS: f64 = 1E-8;

pub struct BatchNormalizeLayer {
    nneurons: usize,
    mean: Vector,
    variance: Vector,
    stddev: Vector,
    one_over_stddev: Vector,
    pub(crate) gamma: Vector,
    beta: Vector,
}

impl BatchNormalizeLayer {
    pub fn new(nneurons: usize) -> BatchNormalizeLayer {
        BatchNormalizeLayer {
            nneurons,
            mean: Vector::new(0),
            variance: Vector::new(0),
            stddev: Vector::new(0),
            one_over_stddev: Vector::new(0),
            gamma: Vector::new(0),
            beta: Vector::new(0),
        }
    }

    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        assert_eq!(self.nneurons, prev_layer.number_of_neurons());
        self.gamma = Vector::from(vec![1.0; self.nneurons]);
        self.beta = Vector::from(vec![0.0; self.nneurons]);
    }

    pub(crate) fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn next_minibatch(&mut self, layer_index: usize, mbs: &[Minibatch]) {
        // SS: calculate mean and variance across minibatch
        self.mean = BatchNormalizeLayer::mean(layer_index, mbs);
        self.variance = BatchNormalizeLayer::variance(layer_index, mbs, &self.mean);
        self.stddev = BatchNormalizeLayer::stddev(&self.variance);
        self.one_over_stddev = BatchNormalizeLayer::one_over_stddev(&self.stddev);
    }

    fn variance(layer_index: usize, mbs: &[Minibatch], means: &Vector) -> Vector {
        assert!(mbs.len() > 0);
        let msize = mbs[0].output[layer_index].dim();
        let mut variance = mbs.iter().fold(Vector::new(msize), |mut accum, mb| {
            let output = &mb.output[layer_index];
            let tmp = output - &means;
            let tmp2 = ops::hadamard(&tmp, &tmp);
            accum += &tmp2;
            accum
        });
        variance /= mbs.len();
        variance
    }

    fn stddev(variance: &Vector) -> Vector {
        ops::f(variance, &f64::sqrt)
    }

    fn one_over_stddev(stddev: &Vector) -> Vector {
        ops::f(stddev, &|x| 1.0 / x)
    }

    fn mean(layer_index: usize, mbs: &[Minibatch]) -> Vector {
        assert!(mbs.len() > 0);
        let msize = mbs[0].output[layer_index].dim();
        let mut means = mbs.iter().fold(Vector::new(msize), |mut accum, mb| {
            let output = &mb.output[layer_index];
            accum += output;
            accum
        });
        means /= mbs.len();
        means
    }

    pub(crate) fn feedforward(&mut self, layer_index: usize, mbs: &mut [Minibatch]) {
        assert!(layer_index > 0);
        self.next_minibatch(layer_index - 1, &mbs);
        for mb in mbs {
            let input = &mb.output[layer_index - 1];
            let x_hat = self.x_hat(input);
            let y = &ops::hadamard(&x_hat, &self.gamma) + &self.beta;
            mb.output[layer_index] = y;
        }
    }

    fn x_hat(&self, z: &Vector) -> Vector {
        assert_eq!(z.dim(), self.mean.dim());
        let tmp1 = z - &self.mean;
        let x_hat = ops::hadamard(&tmp1, &self.one_over_stddev);
        x_hat
    }

    pub(crate) fn backprop(&self, layer_index: usize, mbs: &mut [Minibatch]) {
        assert!(layer_index > 0);
        assert!(!mbs.is_empty());

        let dim = mbs[0].output[layer_index].dim();

        assert_eq!(dim, self.mean.dim());
        assert_eq!(dim, self.variance.dim());
        assert_eq!(dim, self.stddev.dim());
        assert_eq!(dim, self.one_over_stddev.dim());
        assert_eq!(dim, self.gamma.dim());
        assert_eq!(dim, self.beta.dim());

        // SS: for each dimension of the input vector to the BN layer
        for k in 0..dim {
            let sigma2 = self.variance[k];
            let sigma = self.stddev[k];
            let tmp1 = sigma2 + EPS;
            let tmp2 = tmp1.sqrt();
            let tmp3 = 1.0 / (tmp1 * tmp2);
            let tmp4 = -0.5 * tmp3;

            // SS: for each minibatch
            for m in 0..mbs.len() {
                let mut dl_dsigma2 = 0.0;

                // todo SS: use fold here?
                for i in 0..mbs.len() {
                    let dC_dy = mbs[i].error[layer_index + 1][k];
                    let dC_dxhat = dC_dy * self.gamma[k];
                    let x = mbs[i].output[layer_index - 1][k];
                    let tmp = dC_dxhat * (x - self.mean[k]);
                    dl_dsigma2 += tmp;
                }
                dl_dsigma2 *= tmp4;




                let mut dl_dmu = 0.0;
                for i in 0..mbs.len() {
                    let dC_dy = mbs[i].error[layer_index + 1][k];
                    let dC_dxhat = dC_dy * self.gamma[k];
                    dl_dmu += dC_dxhat;
                }
                dl_dmu *= 1.0 / tmp2;

                let mut nom = 0.0;
                for i in 0..mbs.len() {
                    let x = mbs[i].output[layer_index - 1][k];
                    let tmp = -2.0 * (x - self.mean[k]);
                    nom += tmp;
                }
                dl_dmu += dl_dsigma2 * nom / mbs.len() as f64;




                let dC_dy = mbs[m].error[layer_index + 1][k];
                let dC_dxhat = dC_dy * self.gamma[k];

                let t1 = dC_dxhat / tmp2;

                let x = mbs[m].output[layer_index - 1][k];
                let t2 = dl_dsigma2 * 2.0 * (x - self.mean[k]) / mbs.len() as f64;

                let t3 = dl_dmu / mbs.len() as f64;

                let dl_dx = t1 + t2 + t3;

                mbs[m].error[layer_index][k] = dl_dx;
            }
        }
    }

    pub(crate) fn print_summary(&self) {
        println!("{:15} | {:15} | {:15}", "BN", "", "");
    }

    pub(crate) fn update_network(&mut self, layer_index: usize, mbs: &[Minibatch]) {
        // SS: adjust parameters this layer leans, gamma and beta
        let (dgamma, dbeta) = self.calculate_derivatives(layer_index, mbs);

        // update gamma and beta
        self.update_parameters(&dgamma, &dbeta);
    }

    fn calculate_derivatives(&mut self, layer_index: usize, mbs: &[Minibatch]) -> (Vector, Vector) {
        (Vector::new(0), Vector::new(0))
    }

    fn update_parameters(&mut self, dgamma: &Vector, dbeta: &Vector) {}
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;
    use crate::ann::layers::fc_layer::FCLayer;
    use crate::ann::layers::input_layer::InputLayer;
    use crate::ann::layers::layer::Layer::BatchNormalize;

    #[test]
    fn test_means() {
        // Arrange
        let mut mb1 = Minibatch::new(vec![2, 3, 2]);
        let mut mb2 = Minibatch::new(vec![2, 3, 2]);
        let mut mb3 = Minibatch::new(vec![2, 3, 2]);
        let mut mb4 = Minibatch::new(vec![2, 3, 2]);
        let mut mbs = [mb1, mb2, mb3, mb4];

        mbs[0].output[1][0] = 7.0;
        mbs[0].output[1][1] = 9.0;
        mbs[0].output[1][2] = 10.0;

        mbs[1].output[1][0] = 3.0;
        mbs[1].output[1][1] = -2.0;
        mbs[1].output[1][2] = 6.0;

        mbs[2].output[1][0] = 4.0;
        mbs[2].output[1][1] = 3.0;
        mbs[2].output[1][2] = 2.0;

        mbs[3].output[1][0] = 12.0;
        mbs[3].output[1][1] = 13.0;
        mbs[3].output[1][2] = 6.0;

        // Act
        let means = BatchNormalizeLayer::mean(1, &mbs);

        // Assert
        assert_eq!(means.dim(), 3);
        assert_approx_eq!(means[0], 6.5);
        assert_approx_eq!(means[1], 5.75);
        assert_approx_eq!(means[2], 6.0);
    }

    #[test]
    fn test_variance() {
        // Arrange
        let mut mb1 = Minibatch::new(vec![2, 3, 2]);
        let mut mb2 = Minibatch::new(vec![2, 3, 2]);
        let mut mb3 = Minibatch::new(vec![2, 3, 2]);
        let mut mb4 = Minibatch::new(vec![2, 3, 2]);
        let mut mbs = [mb1, mb2, mb3, mb4];

        mbs[0].output[1][0] = 7.0;
        mbs[0].output[1][1] = 9.0;
        mbs[0].output[1][2] = 10.0;

        mbs[1].output[1][0] = 3.0;
        mbs[1].output[1][1] = -2.0;
        mbs[1].output[1][2] = 6.0;

        mbs[2].output[1][0] = 4.0;
        mbs[2].output[1][1] = 3.0;
        mbs[2].output[1][2] = 2.0;

        mbs[3].output[1][0] = 12.0;
        mbs[3].output[1][1] = 13.0;
        mbs[3].output[1][2] = 6.0;

        let means = BatchNormalizeLayer::mean(1, &mbs);

        // Act
        let variance = BatchNormalizeLayer::variance(1, &mbs, &means);

        // Assert
        assert_eq!(variance.dim(), 3);
        assert_approx_eq!(variance[0], 12.25);
        assert_approx_eq!(variance[1], 32.6875);
        assert_approx_eq!(variance[2], 8.0);
    }

    #[test]
    fn test_one_over_stddev() {
        // Arrange
        let mut mb1 = Minibatch::new(vec![2, 3, 2]);
        let mut mb2 = Minibatch::new(vec![2, 3, 2]);
        let mut mb3 = Minibatch::new(vec![2, 3, 2]);
        let mut mb4 = Minibatch::new(vec![2, 3, 2]);
        let mut mbs = [mb1, mb2, mb3, mb4];

        mbs[0].output[1][0] = 7.0;
        mbs[0].output[1][1] = 9.0;
        mbs[0].output[1][2] = 10.0;

        mbs[1].output[1][0] = 3.0;
        mbs[1].output[1][1] = -2.0;
        mbs[1].output[1][2] = 6.0;

        mbs[2].output[1][0] = 4.0;
        mbs[2].output[1][1] = 3.0;
        mbs[2].output[1][2] = 2.0;

        mbs[3].output[1][0] = 12.0;
        mbs[3].output[1][1] = 13.0;
        mbs[3].output[1][2] = 6.0;

        let means = BatchNormalizeLayer::mean(1, &mbs);
        let variance = BatchNormalizeLayer::variance(1, &mbs, &means);
        let stddev = BatchNormalizeLayer::stddev(&variance);

        // Act
        let one_over_stddev = BatchNormalizeLayer::one_over_stddev(&stddev);

        // Assert
        assert_eq!(one_over_stddev.dim(), 3);
        assert_approx_eq!(one_over_stddev[0], 1.0 / 3.5);
        assert_approx_eq!(one_over_stddev[1], 1.0 / 5.7172983130146);
        assert_approx_eq!(one_over_stddev[2], 1.0 / 2.8284271247462);
    }

    #[test]
    fn test_calculate_x_hat() {
        // Arrange
        let mut mb1 = Minibatch::new(vec![2, 3, 2]);
        let mut mb2 = Minibatch::new(vec![2, 3, 2]);
        let mut mb3 = Minibatch::new(vec![2, 3, 2]);
        let mut mb4 = Minibatch::new(vec![2, 3, 2]);
        let mut mbs = [mb1, mb2, mb3, mb4];

        mbs[0].output[1][0] = 7.0;
        mbs[0].output[1][1] = 9.0;
        mbs[0].output[1][2] = 10.0;

        mbs[1].output[1][0] = 3.0;
        mbs[1].output[1][1] = -2.0;
        mbs[1].output[1][2] = 6.0;

        mbs[2].output[1][0] = 4.0;
        mbs[2].output[1][1] = 3.0;
        mbs[2].output[1][2] = 2.0;

        mbs[3].output[1][0] = 12.0;
        mbs[3].output[1][1] = 13.0;
        mbs[3].output[1][2] = 6.0;

        let mut layer = BatchNormalizeLayer::new(3);
        layer.next_minibatch(1, &mbs);
        let z = Vector::from(vec![1.0, 1.0, 1.0]);

        // Act
        let x_hat = layer.x_hat(&z);

        // Assert
        assert_eq!(x_hat.dim(), 3);

        let mean = BatchNormalizeLayer::mean(1, &mbs);
        let variance = BatchNormalizeLayer::variance(1, &mbs, &mean);
        let stddev = BatchNormalizeLayer::stddev(&variance);

        assert_approx_eq!(x_hat[0], (z[0] - mean[0]) / stddev[0]);
        assert_approx_eq!(x_hat[1], (z[1] - mean[1]) / stddev[1]);
        assert_approx_eq!(x_hat[2], (z[2] - mean[2]) / stddev[2]);
    }

    #[test]
    fn test_feedforward1() {
        // Arrange
        let mut mb1 = Minibatch::new(vec![3, 3, 2]);
        let mut mb2 = Minibatch::new(vec![3, 3, 2]);
        let mut mb3 = Minibatch::new(vec![3, 3, 2]);
        let mut mb4 = Minibatch::new(vec![3, 3, 2]);
        let mut mbs = [mb1, mb2, mb3, mb4];

        mbs[0].output[1][0] = 7.0;
        mbs[0].output[1][1] = 9.0;
        mbs[0].output[1][2] = 10.0;

        mbs[1].output[1][0] = 3.0;
        mbs[1].output[1][1] = -2.0;
        mbs[1].output[1][2] = 6.0;

        mbs[2].output[1][0] = 4.0;
        mbs[2].output[1][1] = 3.0;
        mbs[2].output[1][2] = 2.0;

        mbs[3].output[1][0] = 12.0;
        mbs[3].output[1][1] = 13.0;
        mbs[3].output[1][2] = 6.0;

        let prev_layer = FCLayer::new(3);
        let mut layer = BatchNormalizeLayer::new(3);
        layer.initialize(&Layer::from(prev_layer));
        layer.next_minibatch(1, &mbs);

        let z = Vector::from(vec![1.0, 1.0, 1.0]);
        mbs[0].output[0] = z.clone();

        // Act
        layer.feedforward(1, &mut mbs);

        // Assert
        let y = &mbs[0].output[0];
        assert_eq!(y.dim(), 3);

        // SS: gamma=1 and beta=0 initially
        let x_hat = layer.x_hat(&z);
        assert_approx_eq!(x_hat[0], x_hat[0]);
        assert_approx_eq!(x_hat[1], x_hat[1]);
        assert_approx_eq!(x_hat[2], x_hat[2]);
    }

    #[test]
    fn test_backprop_gradient_1() {
        // Arrange
        let input_layer = Layer::from(InputLayer::new(2));
        let mut layer = BatchNormalizeLayer::new(2);
        layer.initialize(&input_layer);

        let mut mbs = [Minibatch::new(vec![2, 2, 2]), Minibatch::new(vec![2, 2, 2])];

        let z00 = 3.0;
        let z01 = 21.0;
        mbs[0].output[0] = Vector::from(vec![z00, z01]);

        let z10 = 4.0;
        let z11 = 11.0;
        mbs[1].output[0] = Vector::from(vec![z10, z11]);

        layer.feedforward(1, &mut mbs);

        // C(y0, y1) = -3 * sin(y0) + 5 * cos(y1)

        // set dC/dy
        let y0 = mbs[0].output[1][0];
        let y1 = mbs[0].output[1][1];
        let dCdy0 = -3.0 * y0.cos();
        let dCdy1 = -5.0 * y1.sin();
        mbs[0].error[2] = Vector::from(vec![dCdy0, dCdy1]);

        let y0 = mbs[1].output[1][0];
        let y1 = mbs[1].output[1][1];
        let dCdy0 = -3.0 * y0.cos();
        let dCdy1 = -5.0 * y1.sin();
        mbs[1].error[2] = Vector::from(vec![dCdy0, dCdy1]);

        // Act
        layer.backprop(1, &mut mbs);

        // Assert

        // dCdz0 = dCda0 * da0dz0 + dCda1 * da1dz0 + dCda2 * da2dz0
//        let dCdz0 = dCda0 * weights[(0, 0)] + dCda1 * weights[(1, 0)] + dCda2 * weights[(2, 0)];
//        let dCdz1 = dCda0 * weights[(0, 1)] + dCda1 * weights[(1, 1)] + dCda2 * weights[(2, 1)];
//        assert_approx_eq!(dCdz0, mbs[0].error[1][0], 1E-12);
//        assert_approx_eq!(dCdz1, mbs[0].error[1][1], 1E-12);

    }
}
