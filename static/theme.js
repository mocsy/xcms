$(document).ready(function () {
    // expand last expanded menu
    let menu_id = '#' + localStorage.getItem('expanded_menu');
    $(menu_id).collapse('show');

    // set active class on links of the current page
    $("a").each(function () {
        if ($(this).attr("href") == window.location.pathname) {
            $(this).addClass("active");
        }
    });

    $('#togglingnavbar .collapse').on('show.bs.collapse', function (obj) {
        // save expanded menu
        localStorage.setItem('expanded_menu', obj.target.id);
    });
});
