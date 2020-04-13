macro_rules! UiElement {
    () => { iced::Element<<$crate::AppUi as iced::Application>::Message> };
    (for<$lf:lifetime>) => { iced::Element<$lf, <$crate::AppUi as iced::Application>::Message> };
}

macro_rules! UiMessage {
    () => { <$crate::AppUi as iced::Application>::Message };
}

// taken from 'iced/example/clock'
pub mod time_utils {
    use iced::futures;

    pub fn every(
        duration: std::time::Duration,
    ) -> iced::Subscription<chrono::DateTime<chrono::Utc>> {
        iced::Subscription::from_recipe(Every(duration))
    }

    struct Every(std::time::Duration);

    impl<H, I> iced_native::subscription::Recipe<H, I> for Every
    where
        H: std::hash::Hasher,
    {
        type Output = chrono::DateTime<chrono::Utc>;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;

            std::any::TypeId::of::<Self>().hash(state);
            self.0.hash(state);
        }

        fn stream(
            self: Box<Self>,
            _input: futures::stream::BoxStream<'static, I>,
        ) -> futures::stream::BoxStream<'static, Self::Output> {
            use futures::stream::StreamExt;

            async_std::stream::interval(self.0)
                .map(|_| chrono::Utc::now())
                .boxed()
        }
    }
}
