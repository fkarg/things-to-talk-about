# Source: https://stackoverflow.com/questions/17834582/run-make-in-each-subdirectory

TOPTARGETS := all clean

SUBDIRS := $(wildcard */.)

$(TOPTARGETS): $(SUBDIRS)
$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)

.PHONY: $(TOPTARGETS) $(SUBDIRS)




# 
# filename="proseminar/präsens"
# 
# all: 
# 	pdflatex --output-directory=proseminar ${filename}.tex
# 	evince ${filename}.pdf &
# 
# pdf: ps
# 	ps2pdf ${filename}.ps
# 
# pdf-print: ps
# 	ps2pdf -dColorConversionStrategy=/LeaveColorUnchanged -dPDFSETTINGS=/printer ${filename}.ps
# 
# text: html
# 	html2text -width 100 -style pretty ${filename}/${filename}.html | sed -n '/./,$$p' | head -n-2 >${filename}.txt
# 
# html:
# 	@#latex2html -split +0 -info "" -no_navigation ${filename}
# 		htlatex ${filename}
# 
# ps:	dvi
# 	dvips -t letter ${filename}.dvi
# 
# dvi:
# 	latex ${filename}
# 	bibtex ${filename}||true
# 	latex ${filename}
# 	latex ${filename}
# 
# read:
# 	evince ${filename}.pdf &
# 
# aread:
# 	acroread ${filename}.pdf
# 
# clean:
# 	rm -f ${filename}.{ps,pdf,log,aux,out,dvi,bbl,blg}
# 
