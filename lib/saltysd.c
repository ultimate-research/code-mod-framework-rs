#include "saltysd_core.h"
#include "saltysd_ipc.h"
#include "saltysd_dynamic.h"

// Ok so like idk what's going on here but all these shims make the real things work so they stay

uint64_t S_getCodeStart() {
	return SaltySDCore_getCodeStart();
}
uint64_t S_getCodeSize() {
	return SaltySDCore_getCodeSize();
}
uint64_t S_findCode(uint8_t* code, size_t size) {
	return SaltySDCore_findCode(code, size);
}

uint64_t S_GetSymbolAddr(void* base, char* name) {
	return SaltySDCore_GetSymbolAddr(base, name);
}
uint64_t S_FindSymbol(char* name) {
	return SaltySDCore_FindSymbol(name);
}
uint64_t S_FindSymbolBuiltin(char* name) {
	return SaltySDCore_FindSymbolBuiltin(name);
}
void S_RegisterModule(void* base) {
	SaltySDCore_RegisterModule(base);
}
void S_RegisterBuiltinModule(void* base) {
	SaltySDCore_RegisterBuiltinModule(base);
}
void S_DynamicLinkModule(void* base) {
	SaltySDCore_DynamicLinkModule(base);
}
void S_ReplaceModuleImport(void* base, char* name, void* new_replace) {
	SaltySDCore_ReplaceModuleImport(base, name, new_replace);
}
void S_ReplaceImport(char* name, void* new_replace) {
	SaltySDCore_ReplaceImport(name, new_replace);
}

void S_Init() {
	SaltySD_Init();
}

Result S_Deinit() {
	return SaltySD_Deinit();
}
Result S_Term() {
	return SaltySD_Term();
}
Result S_Restore() {
	return SaltySD_Restore();
}
Result S_LoadELF(uint64_t heap, uint64_t* elf_addr, uint64_t* elf_size, char* name) {
	return SaltySD_LoadELF(heap, elf_addr, elf_size, name);
}
Result S_Memcpy(uint64_t to, uint64_t from, uint64_t size) {
	return SaltySD_Memcpy(to, from, size);
}
Result S_GetSDCard(Handle *retrieve) {
	return SaltySD_GetSDCard(retrieve);
}
Result S_printf(const char* format, ...) {
	return SaltySD_printf(format);
}
