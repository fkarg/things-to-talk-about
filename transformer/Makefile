# Latex Makefile using latexmk
# Modified by Dogukan Cagatay <dcagatay@gmail.com>
# Modified by Philipp Jund
# Modified by Felix Karg <fkarg10@gmail.com>
# Adapted from : http://tex.stackexchange.com/a/40759
# Additional Source:
# https://serverfault.com/questions/153875/how-to-let-cp-command-dont-fire-an-error-when-source-file-does-not-exist


PROJNAME=main_handout main_present
PDFNAMES=$(addsuffix .pdf, $(PROJNAME))
COMPRESSED=$(addsuffix _compr.pdf, $(PROJNAME))
OUT_DIR=out

.PHONY: fast all compress clean

fast: main_handout.pdf

all: $(PDFNAMES) $(addsuffix .pdfpc, $(PROJNAME))

compress: $(COMPRESSED)


# -dPDFSETTINGS=
# Filesize: /ebook < /printer < /prepress < /default
%_compr.pdf: %.pdf
	gs -sDEVICE=pdfwrite -dNOPAUSE -dQUIET -dPrinted=false -dBATCH -dPDFSETTINGS=/printer -sOutputFile=$@ $<


# *.tex and */*.tex are what files %.pdf depends on. Necessary to notice when
# changes have been made and recomputation is necessary.
%.pdf: %.tex *.tex */*.tex img/*
	mkdir -p $(OUT_DIR)/$*/chapters
	cp ../template/pdfpcnotes.sty $(OUT_DIR)/$*/pdfpcnotes.sty
	latexmk -outdir=$(OUT_DIR)/$* -pdf -use-make -file-line-error -shell-escape $<
	cp -v $(OUT_DIR)/$*/$*.pdf . || :


# '.' prevents of having to specify the complete filename (as we may be copying multiple files)
# '||' is similar to a pipe, but only run if an error happened
# ':' is a posix-'true', preventing it from failing if a file was missing (allowing 'make fast')
# the sed commands transform:
# - newlines (\\) into actual newlines
# - (\par) in two newlines
# - trim whitespaces
%.pdfpc: %.pdf
	cp -v $(OUT_DIR)/$*/$*.pdfpc . || :
	sed -i 's/\\\\/\n/g' $*.pdfpc
	sed -i 's/\\par/\n\n/g' $*.pdfpc
	sed -i "s/^[ \t]*//;s/[ \t]*$$//" $*.pdfpc


clean:
	rm -rf $(OUT_DIR)


dist-clean: clean
	rm -rf $(PDFNAMES)
	rm -rf $(COMPRESSED)
	rm -rf $(addsuffix .pdfpc, $(PROJNAME))
	rm -rf _minted-*
