pub enum IconName {
    Rocket,
    SwitchVertical,
}

pub fn get(icon: IconName) -> &'static str {
    match icon {
        IconName::Rocket => {
            r#"<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-rocket" width="24" height="24" viewBox="0 0 24 24" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
   <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
   <path d="M4 13a8 8 0 0 1 7 7a6 6 0 0 0 3 -5a9 9 0 0 0 6 -8a3 3 0 0 0 -3 -3a9 9 0 0 0 -8 6a6 6 0 0 0 -5 3"></path>
   <path d="M7 14a6 6 0 0 0 -3 6a6 6 0 0 0 6 -3"></path>
   <circle cx="15" cy="9" r="1"></circle>
</svg>"#
        }
        IconName::SwitchVertical => {
            r#"<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-switch-vertical" width="24" height="24" viewBox="0 0 24 24" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
   <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
   <polyline points="3 8 7 4 11 8"></polyline>
   <line x1="7" y1="4" x2="7" y2="13"></line>
   <polyline points="13 16 17 20 21 16"></polyline>
   <line x1="17" y1="10" x2="17" y2="20"></line>
</svg>"#
        }
    }
}
