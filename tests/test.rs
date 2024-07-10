mod hihihaha {

    use casopis::Casopis;
    use log::Level;
    #[test]
    fn test() {
        let _ = Casopis::init(Level::Trace);
        log::trace!("Trace");
        log::debug!("Debug");
        log::info!("Info");
        log::warn!("Warn");
        log::error!("Error");
    }
}
