# Latex Makefile using latexmk
# Modified by Dogukan Cagatay <dcagatay@gmail.com>
# Modified by Philipp Jund
# Modified by Felix Karg <fkarg10@gmail.com>
# Adapted from : http://tex.stackexchange.com/a/40759
# Additional Source:
# https://serverfault.com/questions/153875/how-to-let-cp-command-dont-fire-an-error-when-source-file-does-not-exist


PROJNAME=main_handout main_present
PDFNAMES=$(foreach PDF, $(PROJNAME),$(PDF).pdf)
OUT_DIR=out

.PHONY: $(PDFNAMES) all clean

fast: dir pdfpc main_handout.pdf copy

all: dir pdfpc $(PDFNAMES) copy

dir:
	mkdir -p $(OUT_DIR)/chapters

pdfpc:
	cp ../template/pdfpcnotes.sty out/pdfpcnotes.sty


$(PDFNAMES): %.pdf: %.tex
	latexmk -outdir=$(OUT_DIR) -pdf -use-make -file-line-error -shell-escape $<


# '.' prevents of having to specify the complete filename (as we may be copying multiple files)
# '||' is similar to a pipe, but only run if an error happened
# ':' is a posix-'true', preventing it from failing if a file was missing (allowing 'make fast')
copy:
	cp -uv $(foreach PDF, $(PDFNAMES),$(OUT_DIR)/$(PDF)) . || :
	cp -uv $(foreach PDFPC, $(PROJNAME),$(OUT_DIR)/$(PDFPC).pdfpc) . || :

clean:
	rm -rf $(OUT_DIR)
