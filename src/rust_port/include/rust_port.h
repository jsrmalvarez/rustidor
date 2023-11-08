#ifndef RUSTIDOR
#define RUSTIDOR

extern "C" {

int   rstd_getweapon(const char* c_char);
bool  rstd_iszero(const void* v);
void* rstd_mul(void* v, const float f);

}
#endif//RUSTIDOR
