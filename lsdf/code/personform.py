class OrderSimpleForm(forms.ModelForm):
    p1_email = forms.EmailField(max_length=100, label='Email')
    p1_institute = forms.CharField(max_length=300, required=False, label='Institute')
    p1_organization = forms.CharField(max_length=300, required=False, label='Organization')
    p1_firstname = forms.CharField(max_length=100, required=True, label='Firstname')
    p1_lastname = forms.CharField(max_length=100, required=True, label='Lastname')
    p1_roles = forms.MultipleChoiceField(widget=CheckboxSelectMultipleWithDisabledOption, label='Roles', required=False,
                                         choices=(
                                             ("ROLE_HEAD", "Head of the project"),
                                             ("ROLE_TECH", "Technical contact"),
                                         ))

    p2_email = forms.EmailField(max_length=100, required=False, label='Email')
    p2_institute = forms.CharField(max_length=300, required=False, label='Institute')
    p2_organization = forms.CharField(max_length=300, required=False, label='Organization')
    p2_firstname = forms.CharField(max_length=100, required=False, label='Firstname')
    p2_lastname = forms.CharField(max_length=100, required=False, label='Lastname')
    p2_roles = forms.MultipleChoiceField(widget=forms.CheckboxSelectMultiple, required=False, label='Roles',
                                         choices=(
                                             ("ROLE_HEAD", "Head of the project"),
                                             ("ROLE_TECH", "Technical contact"),
                                         ))
    ...                                  ...
