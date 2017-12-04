test_passphrase <- function(s) {
    parts <- unlist(strsplit(s, ' ', TRUE))
    unique_parts <- unique(parts)
    if (length(unique_parts) != length(parts)) {
        return(FALSE)
    }
    sort_part <- function(part) {
        ls <- unlist(strsplit(part, NULL))
        unique_ls <- sort(unique(ls))
        result <- paste(unique_ls, collapse = "")
        return(result)
    }
    sorted_parts <- unique(lapply(parts, sort_part))
    return(length(sorted_parts) == length(parts))
}
dat <- readLines('input')
valid <- Filter(test_passphrase, dat)
print(length(valid))