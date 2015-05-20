#include <stdlib.h>
#include <groonga/groonga.h>

cgroonga_text_init(grn_obj *text, unsigned char impl_flags) {
  GRN_TEXT_INIT(text, impl_flags);
}

cgroonga_text_put(grn_obj *ctx, grn_obj *text, const unsigned char *str, unsigned int len) {
  GRN_TEXT_PUT(ctx, text, str, len);
}
