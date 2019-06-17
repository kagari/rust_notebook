$(function(){
  function submitCode() {
    var $input = $('.input textarea').val();
    $('.output pre').text('');

    $.ajax({
        url:'http://localhost:8000/api',
        type:'POST',
        data:$input
    })
    // Ajaxリクエストが成功した時発動
    .done( (data) => {
        $('.output pre').html(data.text[0]);
        // console.log(data);
    })
    // Ajaxリクエストが失敗した時発動
    .fail( (data) => {
        $('.output pre').html(data.text[0]);
        // console.log(data);
    })
    // Ajaxリクエストが成功・失敗どちらでも発動
    .always( (data) => {
    });
  }

  $('.input, .input textarea').on('keydown', function(e){
    if(event.shiftKey){
      if(e.keyCode === 13){
        submitCode();
        return false;
      }
    }
  });
  // textareaでTabKeyでインデント出来るようにする．
  $('textarea').on('keydown', function(e){
      if (e.keyCode === 9) {
          e.preventDefault();
          var elem = e.target;
          var val = elem.value;
          var pos = elem.selectionStart;
          var tabLength = 4  // tab length (space)
          elem.value = val.substr(0, pos) + ' '.repeat(tabLength) + val.substr(pos, val.length);
          elem.setSelectionRange(pos + tabLength, pos + tabLength);
      }
  });

  // $('#submit').on('click', function() {
  //   submitCode();
  // });
});
