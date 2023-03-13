library(stackoverflow)
# this is the minimum bound for functionality
n <- 49993
pnts <- lapply(1:n, \(x) runif(2))
stack_overflow_list(pnts)

# there appears to be no such limitation for Doubles
n <- 99999
pnts <- lapply(1:n, \(x) runif(2))
stack_overflow_list_dbls(pnts)

n <- 900000
pnts <- lapply(1:n, \(x) runif(2))
stack_overflow_list_dbls(pnts)


# session will hang after stack overflow so putting at end
# protection stack overflow here
# note that sometimes the maximum n moves by 2-3
n <- 49996
pnts <- lapply(1:n, \(x) runif(2))
stack_overflow_list(pnts)


n <- 50000
pnts <- lapply(1:n, \(x) runif(2))
cpp_stack_overflow(pnts)
