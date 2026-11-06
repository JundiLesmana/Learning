#  Root Makefile — meneruskan perintah ke Makefile di folder python/.
#  Dengan ini kamu bisa langsung dari root repo:
#
#    make run FILE=linear-regression/sp-w8
#    make list
#    make setup
#    make clean
#
#  Setara dengan: make -C python <target> FILE=<file>

.DEFAULT_GOAL := help

help:
	@echo "Perintah yang tersedia:"
	@echo "  make run  FILE=linear-regression/sp-w8  -> jalankan script Python (input interaktif di terminal)"
	@echo "  make list                               -> daftar semua file .py"
	@echo "  make setup                              -> install dependencies Python (sekali saja)"
	@echo "  make clean                              -> hapus __pycache__ & hasil plot"

%:
	@$(MAKE) -C python $@ $(if $(FILE),FILE=$(FILE),)

.PHONY: help