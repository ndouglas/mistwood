#[macro_use]
extern crate mistwood;
use mistwood::messaging::prelude::TemplateProcessor;
use mistwood::messaging::prelude::TemplateProvider;
use mistwood::messaging::stock_templates::goodbye::Goodbye;
use mistwood::messaging::stock_templates::thanks_for_playing::ThanksForPlaying;
use mistwood::messaging::traits::template_processor::TemplateProcessor as TemplateProcessorTrait;

fn main() {
  let mut processor = TemplateProcessor::new();
  let mut provider = TemplateProvider::new();
  processor.set_strict_mode(true);
  let thanks_for_playing = info_message!(provider, ThanksForPlaying, {
    game: "Mistwood".to_string(),
  });
  let goodbye = info_message!(provider, Goodbye);
  println!("{}", processor.process_message(&thanks_for_playing).unwrap());
  println!("{}", processor.process_message(&goodbye).unwrap());
}
