/*
use crate::error::EvaluatorError;
use crate::evaluator::*;
use crate::extractor::FeatureExtractor;
use crate::fit::fit_straight_line;
use crate::float_trait::Float;
use crate::lnerfc::ln_erfc;
use crate::periodogram;

use crate::statistics::Statistics;
use crate::time_series::TimeSeries;

use conv::prelude::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::iter;
use unzip3::Unzip3;
*/
mod amplitude;
pub use amplitude::Amplitude;

mod anderson_darling_normal;
pub use anderson_darling_normal::AndersonDarlingNormal;

mod beyond_n_std;
pub use beyond_n_std::BeyondNStd;

mod bins;
pub use bins::Bins;

mod cusum;
pub use cusum::Cusum;

mod eta;
pub use eta::Eta;

mod eta_e;
pub use eta_e::EtaE;

mod excess_variance;
pub use excess_variance::ExcessVariance;

mod inter_percentile_range;
pub use inter_percentile_range::InterPercentileRange;

mod kurtosis;
pub use kurtosis::Kurtosis;

mod linear_fit;
pub use linear_fit::LinearFit;

mod linear_trend;
pub use linear_trend::LinearTrend;

mod magnitude_percentage_ratio;
pub use magnitude_percentage_ratio::MagnitudePercentageRatio;

mod maximum_slope;
pub use maximum_slope::MaximumSlope;

mod mean;
pub use mean::Mean;

mod mean_variance;
pub use mean_variance::MeanVariance;

mod median;
pub use median::Median;

mod median_absolute_deviation;
pub use median_absolute_deviation::MedianAbsoluteDeviation;

mod median_buffer_range_percentage;
pub use median_buffer_range_percentage::MedianBufferRangePercentage;

mod percent_amplitude;
pub use percent_amplitude::PercentAmplitude;

mod percent_difference_magnitude_percentile;
pub use percent_difference_magnitude_percentile::PercentDifferenceMagnitudePercentile;

mod periodogram;
pub use periodogram::Periodogram;

mod reduced_chi2;
pub use reduced_chi2::ReducedChi2;

mod skew;
pub use skew::Skew;

mod standard_deviation;
pub use standard_deviation::StandardDeviation;

mod stetson_k;
pub use stetson_k::StetsonK;

mod weighted_mean;
pub use weighted_mean::WeightedMean;
