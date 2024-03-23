template_list!(FirstLines, [
  r#"MORK I: The Great Underground Empire
Apologies and thanks to Tim Anderson, Marc Blank, Bruce Daniels, Dave Lebling,
and to everyone else who created the original ZORK and other Infocom games.
ZORK is a registered trademark of Infocom, Inc.
Revision {{ revision }} / Serial number {{ serial_number }}
  "#,
], {
  revision: u8,
  serial_number: u32,
});

template_list!(
  QuitConfirmation,
  ["Do you wish to leave the game? (Y is affirmative): ",]
);
