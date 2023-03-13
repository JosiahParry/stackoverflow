#include <cpp11.hpp>
using namespace cpp11;


list cpp_stack_overflow(list x) {
  int n = x.size();
  writable::list out(n);

  for (int i = 0; i < n; ++i) {
    doubles xi = x[i];
    writable::doubles row = {xi[1], xi[0]};
    out[i] = row;
  }

  return out;
}
