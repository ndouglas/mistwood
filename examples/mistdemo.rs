#[macro_use]
extern crate mistwood;
use mistwood::messaging::_traits::template_processor::TemplateProcessor as TemplateProcessorTrait;
use mistwood::messaging::prelude::TemplateProcessor;
use mistwood::messaging::stock_templates::goodbye::Goodbye;
use mistwood::messaging::stock_templates::thanks_for_playing::ThanksForPlaying;

fn main() {
  let mut processor = TemplateProcessor::new();
  processor.set_strict_mode(true);
  let thanks_for_playing = info_message!(ThanksForPlaying, {
    game: "Mistwood".to_string(),
  });
  let goodbye = info_message!(Goodbye);
  println!("{}", processor.process_message(&thanks_for_playing).unwrap());
  println!("{}", processor.process_message(&goodbye).unwrap());
}
