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
        let xs = mbs.iter().map(|mb| &mb.output[layer_index]).collect::<Vec<_>>();
        let variance = BatchNormalizeLayer::variance_vec(&xs, means);
        variance
    }

    fn variance_vec(xs: &[&Vector], means: &Vector) -> Vector {
        assert!(xs.len() > 0);
        let dim = xs[0].dim();
        let mut variance = xs.iter().fold(Vector::new(dim), |mut accum, &x| {
            let tmp = x - &means;
            let tmp2 = ops::hadamard(&tmp, &tmp);
            accum += &tmp2;
            accum
        });
        variance /= xs.len();
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
        let xs = mbs.iter().map(|mb| &mb.output[layer_index]).collect::<Vec<_>>();
        let mean = BatchNormalizeLayer::mean_vec(&xs);
        mean
    }

    fn mean_vec(xs: &[&Vector]) -> Vector {
        assert!(xs.len() > 0);
        let dim = xs[0].dim();
        let mut mean = xs.iter().fold(Vector::new(dim), |mut accum, &v| {
            accum += v;
            accum
        });
        mean /= xs.len();
        mean
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

    fn x_hat(&self, x: &Vector) -> Vector {
        assert_eq!(x.dim(), self.mean.dim());
        let tmp1 = x - &self.mean;
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
        for i in 0..mbs.len() {
            let sigma2 = &self.variance;
            let tmp2 = sigma2.f(|x| 1.0 / x.sqrt());
            let tmp3 = -0.5 * &tmp2.f(|x| x * x * x);

            let dC_dy = &mbs[i].error[layer_index + 1];
            let dC_dxhat = dC_dy.hadamard(&self.gamma);
            let t1 = dC_dxhat.hadamard(&tmp2);

            let mut dmu = Vector::new(dim);
            let mut dsigma2 = Vector::new(dim);
            for l in 0..mbs.len() {
                let dC_dy = &mbs[l].error[layer_index + 1];
                let dC_dxhat = dC_dy.hadamard(&self.gamma);

                let xi = &mbs[i].output[layer_index - 1];
                let xl = &mbs[l].output[layer_index - 1];

                dmu += &dC_dxhat.hadamard(&tmp2).f(|x| x * (-1.0) / mbs.len() as f64);
                dsigma2 += &dC_dxhat.hadamard(&(xi - &self.mean)).hadamard(&(xl - &self.mean)).hadamard(&tmp3).f(|x| x * 2.0 / mbs.len() as f64);
            }

            let dl_dx = &(&t1 + &dmu) + &dsigma2;
            mbs[i].error[layer_index] = dl_dx;
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
    use linear_algebra::ops::hadamard;

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

        let z = Vector::from(vec![1.0, 6.4, 1.2]);
        mbs[0].output[0] = z.clone();

        // Act
        layer.feedforward(1, &mut mbs);

        // Assert
        let y = &mbs[0].output[1];
        assert_eq!(y.dim(), 3);

        // SS: gamma=1 and beta=0 initially
        let x_hat = layer.x_hat(&z);
        assert_approx_eq!(x_hat[0], y[0], 1E-10);
        assert_approx_eq!(x_hat[1], y[1], 1E-10);
        assert_approx_eq!(x_hat[2], y[2], 1E-10);
    }

    #[test]
    fn test_backprop_gradient_1() {
        // Arrange
        let input_layer = Layer::from(InputLayer::new(2));
        let mut layer = BatchNormalizeLayer::new(2);
        layer.initialize(&input_layer);

        let mut mbs = [Minibatch::new(vec![2, 2, 2]), Minibatch::new(vec![2, 2, 2]), Minibatch::new(vec![2, 2, 2])];

        // set input for minibatch 1
        let x00 = 6.645;
        let x01 = -1.7365;
        mbs[0].output[0] = Vector::from(vec![x00, x01]);

        // set input for minibatch 2
        let x10 = 3.45;
        let x11 = -1.987;
        mbs[1].output[0] = Vector::from(vec![x10, x11]);

        // set input for minibatch 3
        let x20 = 12.45;
        let x21 = -14.987;
        mbs[2].output[0] = Vector::from(vec![x20, x21]);

        layer.feedforward(1, &mut mbs);

        // C(y0, y1) = -3 * sin(y0) + 5 * cos(y1)

        let calculate_dC_dy = |minibatch_idx: usize| -> Vector {
            let y0 = mbs[minibatch_idx].output[1][0];
            let y1 = mbs[minibatch_idx].output[1][1];
            let dCdy0 = -3.0 * y0.cos();
            let dCdy1 = -5.0 * y1.sin();
            Vector::from(vec![dCdy0, dCdy1])
        };

        // set dC/dy for minibatch 1
        let dC_dy1 = calculate_dC_dy(0);

        // set dC/dy for minibatch 2
        let dC_dy2 = calculate_dC_dy(1);

        // set dC/dy for minibatch 3
        let dC_dy3 = calculate_dC_dy(2);

        mbs[0].error[2] = dC_dy1;
        mbs[1].error[2] = dC_dy2;
        mbs[2].error[2] = dC_dy3;

        // Act
        layer.backprop(1, &mut mbs);

        // Assert
        let x_hat = |xs: &[&Vector]| -> Vec<Vector> {
            let mean = BatchNormalizeLayer::mean_vec(xs);
            let variance = BatchNormalizeLayer::variance_vec(xs, &mean);
            let stddev = BatchNormalizeLayer::stddev(&variance);
            let one_over_stddev = BatchNormalizeLayer::one_over_stddev(&stddev);
            let x_hat = xs.iter().map(|&x| (x - &mean).hadamard(&one_over_stddev)).collect::<Vec<_>>();
            x_hat
        };

        let y = |x_hat: f64, gamma: f64, beta: f64| gamma * x_hat + beta;

        let cost_function = |y: &[Vector]| {
            let cost_y0 = -3.0 * y[0][0].sin() + 5.0 * y[0][1].cos();
            let cost_y1 = -3.0 * y[1][0].sin() + 5.0 * y[1][1].cos();
            let cost_y2 = -3.0 * y[2][0].sin() + 5.0 * y[2][1].cos();
            let cost = cost_y0 + cost_y1 + cost_y2;
            cost
        };

        let delta = 1E-6;

        // SS: calc. dC/d x_{0}{0}, i.e. minibatch 0, dim 0
        let c1 = cost_function(&x_hat(&[&vec![x00 + delta, x01].into(), &vec![x10, x11].into(), &vec![x20, x21].into()]));
        let c2 = cost_function(&x_hat(&[&vec![x00 - delta, x01].into(), &vec![x10, x11].into(), &vec![x20, x21].into()]));
        let dC_dx_numeric = (c1 - c2) / 2.0 / delta;
        let dC_dx = mbs[0].error[1][0];
        assert_approx_eq!(dC_dx, dC_dx_numeric, 1E-8);

        // SS: calc. dC/d x_{1}{1}, i.e. minibatch 1, dim 1
        let c1 = cost_function(&x_hat(&[&vec![x00, x01].into(), &vec![x10, x11 + delta].into(), &vec![x20, x21].into()]));
        let c2 = cost_function(&x_hat(&[&vec![x00, x01].into(), &vec![x10, x11 - delta].into(), &vec![x20, x21].into()]));
        let dC_dx_numeric = (c1 - c2) / 2.0 / delta;
        let dC_dx = mbs[1].error[1][1];
        assert_approx_eq!(dC_dx, dC_dx_numeric, 1E-8);

        // SS: calc. dC/d x_{2}{1}, i.e. minibatch 2, dim 1
        let c1 = cost_function(&x_hat(&[&vec![x00, x01].into(), &vec![x10, x11].into(), &vec![x20, x21 + delta].into()]));
        let c2 = cost_function(&x_hat(&[&vec![x00, x01].into(), &vec![x10, x11].into(), &vec![x20, x21 - delta].into()]));
        let dC_dx_numeric = (c1 - c2) / 2.0 / delta;
        let dC_dx = mbs[2].error[1][1];
        assert_approx_eq!(dC_dx, dC_dx_numeric, 1E-8);
    }
}
