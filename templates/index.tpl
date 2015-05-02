<!DOCTYPE html>
<html>
    <head>
        <title>{{title}}</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <!-- Bootstrap -->
        <link href="/vendor/bootstrap/css/bootstrap.min.css" rel="stylesheet">

        <!-- HTML5 Shim and Respond.js IE8 support of HTML5 elements and media queries -->
        <!-- WARNING: Respond.js doesn't work if you view the page via file:// -->
        <!--[if lt IE 9]>
        <script src="https://oss.maxcdn.com/libs/html5shiv/3.7.0/html5shiv.js"></script>
        <script src="https://oss.maxcdn.com/libs/respond.js/1.3.0/respond.min.js"></script>
        <![endif]-->

        <!-- jQuery (necessary for Bootstrap's JavaScript plugins) -->
        <script src="/vendor/jquery-1.9.1.min.js"></script>
        <link href="/styles/style.css" rel="stylesheet">
    </head>
    <body>
        <div class="container">
            <div class="container">
                <div class="jumbotron time-capsule">
                    <div class="container" align="center">
                        <h2>The current local time is {{time}}</h2>
                        <h2>The current time is Moscow, Russia is: {{russian_time}}</h2>
                    </div>
                </div>
            </div>
        </div>
        <!-- Include all compiled plugins (below), or include individual files as needed -->
        <script src="/vendor/bootstrap/js/bootstrap.min.js"></script>
    </body>
</html>
