$("#id_board").change(function () {
  /* on change of `review board`, request new data for `subject area` field */
  var url = $("#form").attr("data-fields-url");     // <host>/ajax/fields/
  var board = $(this).val();    // Which board got selected

  $.ajax({  // request available fields (subject areas) based on selected board
    url: url,       // resolved to <host>/ajax/fields/
    data: { 'board': board },   // currently selected board as part of request
    success: function (data, _textStatus, _jqXHR) {
      $("#id_field").html(data);    // fill in available subject areas
    }
  });
});
