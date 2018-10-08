
$('#list-table tbody').on('click', 'tr', function () {
    let lid = $(this).attr("id");
    let oid = lid.match(/listing-(\d*)/)[1];
    window.location.href = '?id='+oid;
})