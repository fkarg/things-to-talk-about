# Latex Makefile using latexmk
# Modified by Dogukan Cagatay <dcagatay@gmail.com>
# Modified by Philipp Jund
# Originally from : http://tex.stackexchange.com/a/40759


PROJNAME=main
OUT_DIR=out
.PHONY: $(PROJNAME).pdf all clean

all: dir $(PROJNAME).pdf

dir:
	mkdir -p $(OUT_DIR)

$(PROJNAME).pdf: $(PROJNAME).tex 
	pdflatex -output-directory=$(OUT_DIR) -shell-escape $<
	# latexmk -outdir=$(OUT_DIR) -pdf -use-make -file-line-error -shell-escape $<
	# latexmk -outdir=$(OUT_DIR) -file-line-error -shell-escape $<
	cp $(OUT_DIR)/$(PROJNAME).pdf $(PROJNAME).pdf

clean:
	rm -rf $(OUT_DIR)
