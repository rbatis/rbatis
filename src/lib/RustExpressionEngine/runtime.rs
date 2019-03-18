pub fn IsNumber(arg: &String) -> bool{
    let chars = arg.chars();
    for item in chars {
        if item == '.' ||
            item == '0' ||
            item == '1' ||
            item == '2' ||
            item == '3' ||
            item == '4' ||
            item == '5' ||
            item == '6' ||
            item == '7' ||
            item == '8' ||
            item == '9'
        {
            return true;
        }
    }
    return false;
}