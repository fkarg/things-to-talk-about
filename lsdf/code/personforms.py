class PersonForm(forms.ModelForm):
    roles = forms.MultipleChoiceField(
        label="Roles", required=False,
        choices=(
            ("ROLE_HEAD", "Head of the project"),
            ("ROLE_TECH", "Technical contact"),
        ))
    class Meta:
        model = Person
        fields = [ "first_name", "last_name", "email",
            "institute", "roles", "organization",
            ]

PersonFormSet = formset_factory(PersonForm)

class OrderSimpleForm(forms.ModelForm):
    """ Form for Project requests. """
    owner = OwnerForm()  # main contact responsible
    additional_contacts = PersonFormSet({
        'form-TOTAL_FORMS': '0',
        'form-INITIAL_FORMS': '0',
    })
