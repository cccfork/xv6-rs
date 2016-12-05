#include "types.h"
#include "param.h"
#include "memlayout.h"
#include "mmu.h"
#include "proc.h"
#include "defs.h"
#include "x86.h"
#include "elf.h"

void
exec_init(const char *name, pde_t *pgdir, uint sz, const struct elfhdr *elf, uint sp) {
  // Save program name for debugging.
  safestrcpy(get_proc()->name, name, sizeof(get_proc()->name));

  // Commit to the user image.
  pde_t *oldpgdir = get_proc()->pgdir;
  get_proc()->pgdir = pgdir;
  get_proc()->sz = sz;
  get_proc()->tf->eip = elf->entry;  // main
  get_proc()->tf->esp = sp;
  switchuvm(get_proc());
  freevm(oldpgdir);
}
