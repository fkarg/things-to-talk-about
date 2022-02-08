$("#id_board").change(function () {
  /* on change of board field, request new data for `field` field */
  var url = $("#form").attr("data-fields-url");  // <host>/ajax/fields/
  var board = $(this).val();    // Which board got selected

  $.ajax({  // request available fields based on selected board
    url: url,
    data: { 'board': board },   // send board as part of request
    success: function (data, textStatus, jqXHR) {
      $("#id_field").html(data);    // fill in available fields
    }
  });
});
