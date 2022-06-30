# Latex Makefile using latexmk
# Modified by Dogukan Cagatay <dcagatay@gmail.com>
# Modified by Philipp Jund
# Modified by Felix Karg <fkarg10@gmail.com>
# Adapted from : http://tex.stackexchange.com/a/40759
# Additional Source:
# https://serverfault.com/questions/153875/how-to-let-cp-command-dont-fire-an-error-when-source-file-does-not-exist


PROJNAME=main_handout main_present
PDFNAMES=$(addsuffix .pdf, $(PROJNAME))
COMPRESSED=$(addsuffix .compressed.pdf, $(PROJNAME))
OUT_DIR=out

.PHONY: fast all watch compress clean dist-clean

fast: main_handout.pdf main_handout.pdfpc

all: $(PDFNAMES) $(addsuffix .pdfpc, $(PROJNAME))

compress: $(COMPRESSED)

# '.' prevents of having to specify the complete filename (as we may be copying multiple files)
# '||' is similar to a pipe, but only run if an error happened
# ':' is a posix-'true', preventing it from failing if a file was missing (allowing 'make fast')
# %.pdf: %.tex *.tex */*.tex references.bib img/*
%.pdf: %.tex *.tex */*.tex references.bib img/*
	mkdir -p $(OUT_DIR)/$*/chapters/
	latexmk -outdir=$(OUT_DIR)/$* -pdf -use-make -file-line-error -shell-escape $< || :
	# lualatex --file-line-error --output-directory=$(OUT_DIR)/$* --interaction=nonstopmode $< || :
	cp -v $(OUT_DIR)/$*/$*.pdf . || :


# the sed commands transform:
# - newlines (\\) into actual newlines
# - (\par) in two newlines
# - trim whitespaces
%.pdfpc: %.pdf
	cp -v $(OUT_DIR)/$*/$*.pdfpc . || :
	sed -i 's/\\\\/\n/g' $*.pdfpc
	sed -i 's/\\par/\n\n/g' $*.pdfpc
	sed -i "s/^[ \t]*//;s/[ \t]*$$//" $*.pdfpc

watch:
	latexmk -pvc -outdir=$(OUT_DIR)/$* -pdf -use-make -file-line-error -shell-escape $(PROJNAME).tex

# -dPDFSETTINGS=
# Filesize: /ebook < /printer < /prepress < /default
%.compressed.pdf: %.pdf
	gs -sDEVICE=pdfwrite -dNOPAUSE -dQUIET -dPrinted=false -dBATCH -dPDFSETTINGS=/default -sOutputFile=$@ $<
	cp $@ $<

dist-clean: clean
	rm -rf $(PDFNAMES)
	rm -rf $(COMPRESSED)

clean:
	rm -rf $(OUT_DIR)
	rm -rf *.out *.log *.aux
