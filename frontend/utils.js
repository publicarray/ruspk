export function formatBoolean(data) {
    if ( data === true ) {
        return "Yes"
    } else if (data === false) {
        return "No"
    }
    return "-"
}
