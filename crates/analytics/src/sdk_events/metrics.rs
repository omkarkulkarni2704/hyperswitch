use api_models::analytics::{
    sdk_events::{
        SdkEventDimensions, SdkEventFilters, SdkEventMetrics, SdkEventMetricsBucketIdentifier,
    },
    Granularity, TimeRange,
};
use time::PrimitiveDateTime;

use crate::{
    query::{Aggregate, GroupByClause, ToSql, Window},
    types::{AnalyticsCollection, AnalyticsDataSource, LoadRow, MetricsResult},
};

mod authentication_unsuccessful_count;
mod average_payment_time;
mod payment_attempts;
mod payment_data_filled_count;
mod payment_method_selected_count;
mod payment_methods_call_count;
mod sdk_initiated_count;
mod sdk_rendered_count;
mod three_ds_challenge_flow_count;
mod three_ds_frictionless_flow_count;
mod three_ds_method_invoked_count;
mod three_ds_method_skipped_count;
mod three_ds_method_successful_count;
mod three_ds_method_unsuccessful_count;

use authentication_unsuccessful_count::AuthenticationUnsuccessfulCount;
use average_payment_time::AveragePaymentTime;
use payment_attempts::PaymentAttempts;
use payment_data_filled_count::PaymentDataFilledCount;
use payment_method_selected_count::PaymentMethodSelectedCount;
use payment_methods_call_count::PaymentMethodsCallCount;
use sdk_initiated_count::SdkInitiatedCount;
use sdk_rendered_count::SdkRenderedCount;
use three_ds_challenge_flow_count::ThreeDsChallengeFlowCount;
use three_ds_frictionless_flow_count::ThreeDsFrictionlessFlowCount;
use three_ds_method_invoked_count::ThreeDsMethodInvokedCount;
use three_ds_method_skipped_count::ThreeDsMethodSkippedCount;
use three_ds_method_successful_count::ThreeDsMethodSuccessfulCount;
use three_ds_method_unsuccessful_count::ThreeDsMethodUnsuccessfulCount;

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub struct SdkEventMetricRow {
    pub total: Option<bigdecimal::BigDecimal>,
    pub count: Option<i64>,
    pub time_bucket: Option<String>,
    pub payment_method: Option<String>,
    pub platform: Option<String>,
    pub browser_name: Option<String>,
    pub source: Option<String>,
    pub component: Option<String>,
    pub payment_experience: Option<String>,
}

pub trait SdkEventMetricAnalytics: LoadRow<SdkEventMetricRow> {}

#[async_trait::async_trait]
pub trait SdkEventMetric<T>
where
    T: AnalyticsDataSource + SdkEventMetricAnalytics,
{
    async fn load_metrics(
        &self,
        dimensions: &[SdkEventDimensions],
        publishable_key: &str,
        filters: &SdkEventFilters,
        granularity: &Option<Granularity>,
        time_range: &TimeRange,
        pool: &T,
    ) -> MetricsResult<Vec<(SdkEventMetricsBucketIdentifier, SdkEventMetricRow)>>;
}

#[async_trait::async_trait]
impl<T> SdkEventMetric<T> for SdkEventMetrics
where
    T: AnalyticsDataSource + SdkEventMetricAnalytics,
    PrimitiveDateTime: ToSql<T>,
    AnalyticsCollection: ToSql<T>,
    Granularity: GroupByClause<T>,
    Aggregate<&'static str>: ToSql<T>,
    Window<&'static str>: ToSql<T>,
{
    async fn load_metrics(
        &self,
        dimensions: &[SdkEventDimensions],
        publishable_key: &str,
        filters: &SdkEventFilters,
        granularity: &Option<Granularity>,
        time_range: &TimeRange,
        pool: &T,
    ) -> MetricsResult<Vec<(SdkEventMetricsBucketIdentifier, SdkEventMetricRow)>> {
        match self {
            Self::PaymentAttempts => {
                PaymentAttempts
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::PaymentMethodsCallCount => {
                PaymentMethodsCallCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::SdkRenderedCount => {
                SdkRenderedCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::SdkInitiatedCount => {
                SdkInitiatedCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::PaymentMethodSelectedCount => {
                PaymentMethodSelectedCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::PaymentDataFilledCount => {
                PaymentDataFilledCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::AveragePaymentTime => {
                AveragePaymentTime
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::ThreeDsMethodSkippedCount => {
                ThreeDsMethodSkippedCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::ThreeDsMethodInvokedCount => {
                ThreeDsMethodInvokedCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::ThreeDsMethodSuccessfulCount => {
                ThreeDsMethodSuccessfulCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::ThreeDsMethodUnsuccessfulCount => {
                ThreeDsMethodUnsuccessfulCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::AuthenticationUnsuccessfulCount => {
                AuthenticationUnsuccessfulCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::ThreeDsChallengeFlowCount => {
                ThreeDsChallengeFlowCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
            Self::ThreeDsFrictionlessFlowCount => {
                ThreeDsFrictionlessFlowCount
                    .load_metrics(
                        dimensions,
                        publishable_key,
                        filters,
                        granularity,
                        time_range,
                        pool,
                    )
                    .await
            }
        }
    }
}
