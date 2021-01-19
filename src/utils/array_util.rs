pub fn is_array(type_name: &str) -> bool {
    if type_name.starts_with("alloc::collections::linked_list")
        || type_name.starts_with("alloc::vec::Vec<")
        || type_name.starts_with("[")
        || type_name.starts_with("&[")
    {
        return true;
    }
    return false;
}
