use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mistwood::messaging::prelude::{Gravity, Message, TemplateProcessor};
use serde_json::json;

fn string_templating(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("String Templating");
  let gravity = Gravity::Info;
  let message_data = vec![
    ("Hello, null!", json!(null)),
    ("Hello, empty object!", json!({})),
    ("Hello, {{name}}", json!({"name": "world"})),
    ("Goodbye, {{name}}", json!({"name": "world"})),
    (
      "The {{animal}} {{verb}} the {{object}}.",
      json!({"animal":"dog", "verb":"ate", "object":"food"}),
    ),
    (
      "{{article}} {{animal}} {{traveled}} {{direction}} to a {{business.small}}.",
      json!({"article":"A", "animal":"duck", "traveled":"walked", "direction":"up", "business":{"small":"lemonade stand"}}),
    ),
  ];
  let metadata = None;
  let template_processor = TemplateProcessor::new();
  for i in message_data.iter() {
    group.bench_with_input(BenchmarkId::new("Message Creation and Templating", i.0), i, |b, i| {
      b.iter(|| {
        let message = Message {
          template: i.0.to_string(),
          gravity,
          data: i.1.clone(),
          metadata: metadata.clone(),
        };
        template_processor
          .render_template(&message.template, &message.data)
          .unwrap()
      })
    });
    group.bench_with_input(BenchmarkId::new("Message Templating", i.0), i, |b, i| {
      let message = Message {
        template: i.0.to_string(),
        gravity,
        data: i.1.clone(),
        metadata: metadata.clone(),
      };
      b.iter(|| {
        template_processor
          .render_template(&message.template, &message.data)
          .unwrap()
      })
    });
  }
  group.finish();
}

criterion_group!(benches, string_templating);
criterion_main!(benches);
