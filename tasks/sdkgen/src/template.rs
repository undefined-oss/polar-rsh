use anyhow::Result;
use handlebars::Handlebars;

use crate::bindings::{Resources, Webhooks};

pub struct Templates {
    registry: Handlebars<'static>,
}

impl Templates {
    pub fn new() -> Result<Self> {
        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);
        registry.set_strict_mode(true);
        registry
            .register_template_string("resources", include_str!("../templates/resources.rs.hbs"))?;
        registry.register_template_string(
            "webhooks",
            include_str!("../templates/webhook_events.rs.hbs"),
        )?;
        Ok(Self { registry })
    }

    pub fn resources(&self, context: &Resources) -> Result<String> {
        Ok(self.registry.render("resources", context)?)
    }

    pub fn webhooks(&self, context: &Webhooks) -> Result<String> {
        Ok(self.registry.render("webhooks", context)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::{Event, Webhooks};

    #[test]
    fn webhook_template_generates_typed_dispatch() {
        let output = Templates::new()
            .unwrap()
            .webhooks(&Webhooks {
                events: vec![Event {
                    name: "order.created".into(),
                    schema: "WebhookOrderCreatedPayload".into(),
                    variant: "OrderCreated".into(),
                }],
            })
            .unwrap();
        assert!(output.contains("OrderCreated(Box<WebhookOrderCreatedPayload>)"));
        assert!(output.contains(
            "Some(\"order.created\") => \
             serde_json::from_value::<WebhookOrderCreatedPayload>(value)"
        ));
        assert!(output.contains("super::WebhookError::UnknownType"));
        assert!(!output.contains("&lt;"));
    }
}
