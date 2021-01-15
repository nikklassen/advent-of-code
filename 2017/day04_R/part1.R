test_passphrase <- function(s) {
    parts <- unlist(strsplit(s, ' ', TRUE))
    unique_parts <- unique(parts)
    return(length(unique_parts) == length(parts))
}
dat <- readLines('input')
valid <- Filter(test_passphrase, dat)
print(length(valid))