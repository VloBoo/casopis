mod hihihaha {

    use casopis::Casopis;
    use log::Level;
    #[test]
    fn test() {
        let _ = Casopis::init(Level::Trace);
        log::trace!("T");
        log::debug!("D");
        log::info!("I");
        log::warn!("W");
        log::error!("E");
    }
}
