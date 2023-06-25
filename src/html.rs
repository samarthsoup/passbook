pub const HOME: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>passbook</title>

  <style>
    .center{
      position:relative;
      width:100%;
      height:2em;
    }
    .center a{
      position:absolute;
      top:50%;
      left:50%;
      transform: translate(-50%,-50%);
    }
    
  </style>

</head>

<body style="background-color:#121212; font-family:helvetica;" link="ivory;" alink="cornsilk" vlink="ivory">
<br><br><br><br><br><br><br><br><br><br><br><br><br><br>
    <p style="text-align:center; font-size:50px; color: cornsilk;"><b>HOMEPAGE</b></p>
    <div class="center">
    <a href="/login" style="text-decoration:none;">Login</a><br>
    </div>
    <div class="center">
    <a href="/signup" style="text-decoration:none;">Sign Up</a><br><br><br>
    </div>
</body>
</html>
"#;

pub const SIGNUP: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>new account</title>

  <style>
    .form{
      position:relative;
      width:100%;
      height:1em;
    }
    .form form{
      position:absolute;
      top:50%;
      left:50%;
      transform: translate(-50%,-50%);
    }
    
  </style>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
<br><br><br><br><br><br><br><br><br><br><br><br><br><br>
    <h1 style="text-align:center;">add account</h1>
    <br><br>
    <div class="form">
    <form action = "/signupactivity" method = "post">
        <label for="id">account id:</label><br>
        <input type = "number" name = "id" min = "1" style="border:none; border-bottom:2px solid #94B3FD; background-color:black; color:#94B3FD; outline:none"><br>
        <label for="name">name:</label><br>
        <input type = "text" name = "name" style="border:none; border-bottom:2px solid #94B3FD; background-color:black; color:#94B3FD; outline:none"><br>
        <input type = "submit" value = "submit" style="background-color:black; border:none; color:black;">
    </form>
    </div>
</body>
</html>
"#;

pub const SIGNUPFAILURE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>failed</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <p>account number already exists</p>
    <p><a href="/signup" style="text-decoration:none;">try again</a></p>

</body>
</html>
"#;

pub const SIGNUPSUCCESS: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>success</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <p>account added successfully</p>
    <p><a href="/login" style="text-decoration:none;">login</a></p>
</body>
</html>
"#;

pub const LOGIN: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>login</title>

  <style>
    .form{
      position:relative;
      width:100%;
      height:1em;
    }
    .form form{
      position:absolute;
      top:50%;
      left:50%;
      transform: translate(-50%,-50%);
    }
    
  </style>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <br><br><br><br><br><br><br><br><br><br><br><br><br><br>
    <h1 style="text-align:center;">login</h1>
    <br>
    <div class="form">
    <form action = "/loginactivity" method = "post">
        <label for="id">account id:</label><br>
        <input type = "number" name = "id" min = "1" style="border:none; border-bottom:2px solid #94B3FD; background-color:black; color:#94B3FD; outline:none"><br>
        <input type = "submit" value = "submit" style="background-color:black; border:none; color:black;">
    </form>
    </div>
</body>
</html>
"#;

pub const LOGINACTIVITY: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; URL=/userpage/{{ userid }}" />

  <title>loading</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
</body>
</html>
"#;

pub const LOGINFAILURE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>failed</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <p>account does not exist</p>
    <p><a href="/login" style="text-decoration:none;">login</a></p>
    <p><a href="/signup" style="text-decoration:none;">sign up</a></p>
</body>
</html>
"#;

pub const USERPAGE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>{{ user.name }}'s dashboard</title>
  <script type="text/javascript" src="https://www.gstatic.com/charts/loader.js"></script>

  <style>
    .right{
      position:relative;
      width:100%;
      height:2em;
    }
    .right a{
      position:absolute;
      top:0%;
      right:0%;
      transform: translate(0%,0%);
      text-decoration:none;
    }
  </style>

</head>

<body style="background-color:#AFF14C; font-family:helvetica; color:#314139; margin:60px;" link="01BF71" alink="01BF71" vlink="01BF71">
    <h1 style="color:#01BF71;">welcome {{ user.name }}</h1>

    userid: {{ user.acc_no }}<br>
    available balance: {{ user.balance }}<br><br><br>

    <table align="center">
    <tr>
    <td>
    <h2 style="color:#01BF71;">actions:</h2>
    <a href="/deposit/{{ user.acc_no }}" style="text-decoration:none;">deposit</a><br><br>
    <a href="/withdraw/{{ user.acc_no }}" style="text-decoration:none;">withdraw</a><br><br>
    <a href="/assign/{{ user.acc_no }}" style="text-decoration:none;">assign your money</a>
    </td>

    <script type="text/javascript">
      // Load google charts
      google.charts.load('current', {'packages':['corechart']});
      google.charts.setOnLoadCallback(drawChart);

      function drawChart() {
        var data = google.visualization.arrayToDataTable([
        ['categories', 'amount'],
        ['bills', {{user.spending.bills}}],
        ['food', {{user.spending.food}}],
        ['vacation', {{user.spending.vacation}}],
        ['misc', {{user.spending.misc}}],
      ]);

        var options = {
          'width':800, 
          'height':450, 
          'colors': ['#B983FF', '#94B3FD', '#94DAFF', '#99FEFF'],
          'backgroundColor':'AFF14C',
          'fontName': 'helvetica', 'fontSize':14,
          'legend': {'textStyle': {'color':'white'}},
          'title': 'current spending',
          'titleTextStyle': {'color': 'black'}
        };

        var chart = new google.visualization.PieChart(document.getElementById('piechart'));
        chart.draw(data, options);
      }
    </script>

    <td>
    <div id="piechart" align="center"></div>
    </td>
    </tr>
    <tr>
    <td>
    <h2 style="text-align:right;color:#01BF71;">recent transaction history</h2>
                <p style="text-align:right; color:#314139;">
                {% set counter = 0 %}
                    {% for transaction in user.history %}
                      {% if counter < 5 %}
                      {% set counter = counter + 1 %}
                        {{ transaction.amount }}  {{ transaction.crdr }}<br>
                        {{ transaction.date }}<br>
                        {{ transaction.category }}<br>
                        -------------------------------------------------------------------
                        <br>
                      {% endif %}
                    {% endfor %}
                </p>
    <br>
    <div class="right">
    <a href="/history/{{ user.acc_no }}" style="text-decoration:none;">full transaction history</a>
    </td>
    </table>
    <br><br><br><br><br><br><br><br><br>
    </div>
    <p><a href="/delete/{{ user.acc_no }}" style="text-decoration:none;">delete account</a><p>
</body>
</html>
"#;

pub const HISTORY: &str = r#"
<!doctype html>
<html>
        <head>
            <title>user history</title>
        </head>
          <body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
            <h1>transactions</h1>
            
                <p>
                    {% for transaction in user.history %}
                        {{ transaction.amount }}  {{ transaction.crdr }}<br>
                        {{ transaction.date }}<br>
                        {{ transaction.category }}<br>
                        <br>
                        -----------------------------------------------------------------------
                        <br><br>
                    {% endfor %}
                </p>
            
            <p><a href="/userpage/{{ user.acc_no }}" style="text-decoration:none;">back to main menu</a></p>
        </html>
"#;

pub const DEPOSIT: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>deposit page</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <h1>deposit</h1>
    <form action = "/depositactivity/{{ user.acc_no }}" method = "post">
        <input type="hidden" id = "id" name="id" value="{{ user.acc_no }}">
        <label for="amount">amount to be deposited:</label><br>
        <input type = "number" step = "0.01" name = "amount" min = "0" style="border:none; border-bottom:1px solid #94B3FD; background-color:black; color:#94B3FD; outline:none"><br>
        <label for="category">category:</label><br>
        <select name="category" id="category" style="background-color:black;border:none;color:#94B3FD;outline:none;">
          <option value="salary">salary</option>
          <option value="misc">misc</option>
        </select>
        <input type = "submit" value = "submit" style="background-color:black; border:none; color:black;">
    </form>
</body>
</html>
"#;

pub const WITHDRAW: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>withdraw page</title>
</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <h1>withdraw</h1>
    <form action = "/withdrawactivity/{{ user.acc_no }}" method = "post">
        <input type="hidden" id = "id" name="id" value="{{ user.acc_no }}">
        <label for="amount">amount to be withdrawn:</label><br>
        <input type = "number" step = "0.01" name = "amount" min = "0" style="border:none; border-bottom:1px solid #94B3FD; background-color:black; color:#94B3FD; outline:none"><br><br>
        <label for="category">category:</label><br>
        <select name="category" id="category" style="background-color:black;border:none;color:#94B3FD;outline:none;">
          <option value="bills">bills</option>
          <option value="food">food</option>
          <option value="vacation">vacation</option>
          <option value="misc">misc</option>
        </select>
        <input type = "submit" value = "submit" style="background-color:black; border:none; color:black;">
    </form>
</body>
</html>
"#;

pub const ASSIGN: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>budget page</title>
</head>
<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <h1>budget</h1>
    <br><br>
    available money: {{user.balance}}
    <br><br><br><br>
    <form action = "/assignactivity/{{ user.acc_no }}" method = "post">
        <input type="hidden" id = "id" name="id" value="{{ user.acc_no }}">
        <label for="category">category:</label><br>
        <select name="category" id="category" style="background-color:black;border:none;color:#94B3FD;outline:none;">
          <option value="bills">bills</option>
          <option value="food">food</option>
          <option value="vacation">vacation</option>
          <option value="misc">misc</option>
        </select><br>
        <label for="amount">assign money:</label><br>
        <input type = "number" step = "0.01" name = "amount" min = "0" style="border:none; border-bottom:1px solid #94B3FD; background-color:black; color:#94B3FD; outline:none"><br><br>
        <input type = "submit" value = "submit" style="background-color:black; border:none; color:black;">
    </form>
</body>
</html>
"#;

pub const ASSIGNSUCCESS: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta http-equiv="refresh" content="0; url = /assign/{{user.acc_no}}" />
  <title>success</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <p>assigned successfully</p>
</body>
</html>
"#;

pub const SUCCESS: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>success</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <p>transaction was successful</p>
    <p><a href="/userpage/{{ user.acc_no }}" style="text-decoration:none;">back to main menu</a></p>

</body>
</html>
"#;

pub const FAILURE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>failed</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <p>not enough balance</p>
    <p><a href="/userpage/{{ userid }}" style="text-decoration:none;">back to main menu</a></p>

</body>
</html>
"#;

pub const REMOVESUCCESS: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>success</title>

</head>

<body style="background-color:black; font-family:helvetica; color:#94B3FD;" link="99FEFF" alink="94DAFF" vlink="99FEFF">
    <p>account removed successfully</p>
    <p><a href="/" style="text-decoration:none;">back to main menu</a></p>
</body>
</html>
"#;
