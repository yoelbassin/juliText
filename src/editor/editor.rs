use crate::gui::Gui;




struct NewEditor {
    gui: Box< dyn Gui>,
    status_bar: StatusBar,
}