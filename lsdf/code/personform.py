class OrderSimpleForm(ModelForm):
    """ Form for Project requests: <host>/new/ """
    p1_email = EmailField(...)
    p1_institute = CharField(...)
    p1_organization = CharField(...)
    p1_firstname = CharField(...)
    p1_lastname = CharField(...)
    p1_roles = MultipleChoiceField(...)

    p2_email = EmailField(...)
    p2_institute = CharField(...)
    p2_organization = CharField(...)
    p2_firstname = CharField(...)
    p2_lastname = CharField(...)
    p2_roles = MultipleChoiceField(...)

    p3_email = EmailField(...)
    p3_institute = CharField(...)
    p3_organization = CharField(...)
    p3_firstname = CharField(...)
    p3_lastname = CharField(...)
    p3_roles = MultipleChoiceField(...)
    ...
