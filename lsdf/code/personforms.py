class PersonForm(ModelForm):
    roles = MultipleChoiceField(
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

class OrderSimpleForm(ModelForm):
    """ Form for Project requests: <host>/new/ """
    owner = PersonForm()  # main contact responsible
    additional_contacts = PersonFormSet(data={
        'form-TOTAL_FORMS': '0',    # empty form has zero total forms
        'form-INITIAL_FORMS': '0',  # initially no form is shown
    })
