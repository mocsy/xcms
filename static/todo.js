window.onload = function() {
  updateCounter();
};
var exampleSocket = new WebSocket("wss://ecs.dev.reedwolf.com:3000");

exampleSocket.onopen = function (event) {

  exampleSocket.onmessage = function (event) {
    let data = JSON.parse(event.data);
    var contents = $(`#switch${data.id}`).prop('checked', data.value === true);
    $(`#switch${data.id}`).prop("disabled", true);
    setTimeout(() => $(`#switch${data.id}`).prop("disabled", false), 1000);
    updateCounter();
  }

  exampleSocket.onclose = function () {
    window.location.reload(true);
  };
};

function checkboxClicked(id) {
  let value = $(`#switch${id}`).is(':checked');
  let data = JSON.stringify({
    id,
    value
  });
  exampleSocket.send(data);
  var jqxhr = $.post('./todo', { id, value })
  //   .done(function () {
  //     exampleSocket.send(id);
  //   })
  .fail(function () {
    window.location.reload(true);
  })
  //   .always(function () {
  //     console.log("finished");
  //   });
}

function updateCounter() {
  let total = $(".material-switch > input[type='checkbox']").length
  let on = $(".material-switch > input[type='checkbox']:checked").length
  let perc = Math.floor(on/total*100);
  $('#completedcounter').html(on+"/"+total+" "+perc+"%");
}
function textTyped() {
  var value = $("#searchthis").val().toLowerCase();
  $(".searchable").filter(function() {
    console.log(this);
    $(this).toggle($(this).text().toLowerCase().indexOf(value) > -1)
  });
}

//firefox fix
window.onbeforeunload = function() {
  exampleSocket.onclose = function () {}; // disable onclose handler first
  exampleSocket.close();
};
