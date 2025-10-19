// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Moew72 <Moew72@proton.me>

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dlfcn.h>

#ifndef __USE_GNU
#define __USE_GNU
#endif
#include <link.h>

typedef long long (*func)(char*, unsigned char*, int, int, unsigned char*);

char* module_path;
uintptr_t offset;
func sign;

static uintptr_t module_base;
static void* module;

int callback(struct dl_phdr_info* info, size_t, void*) {
  if (info->dlpi_name && strstr(info->dlpi_name, "wrapper.node")) {
    module_base = info->dlpi_addr;
    printf("Found wrapper.node at base: 0x%lx\n", module_base);
    return 1;
  }
  return 0;
}

int load_module() {
  module = dlopen(module_path, RTLD_LAZY);
  if (!module) {
    fprintf(stderr, "dlopen failed: %s\n", dlerror());
    return 1;
  }

  printf("Module handle: %p\n", module);

  dl_iterate_phdr(callback, NULL);

  if (module_base == 0) {
    fprintf(stderr, "Failed to find module base\n");
    dlclose(module);
    return 1;
  }

  sign = (func)(module_base + offset);

  printf("Calculated function address: %p\n", (void*)sign);

  if ((uintptr_t)sign < 0x10000) {
    fprintf(stderr, "Invalid function pointer: %p\n", (void*)sign);
    dlclose(module);
    return 1;
  }

  return 0;
}

void unload_module() {
  dlclose(module);
}
