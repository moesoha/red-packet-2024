<?php
include_once './news.inc.php';
$id = (int)$_GET['id'];
$news = NEWS[$id] ?? null;
if(empty($news)) {
	http_response_code(404);
	echo '<div style="text-align: center;"><h1>404 Not Found</h1><hr><small>SohaHongbao/2.024</small></div>';
	exit(0);
}
if(($news['internal'] ?? false) && (($_SERVER['HTTP_HOST'] ?? '') !== INTERNAL_HOST)) {
	http_response_code(302);
	$url = 'https://'.INTERNAL_HOST.$_SERVER['REQUEST_URI'];
	header("Location: $url");
	echo '<a href="'.htmlspecialchars($url).'">点此重定向到内网查阅。</a>';
	exit(0);
}
// if(!empty($allowNet = $news['ip'] ?? [])) {
// 	$ip = inet_pton($_SERVER['REMOTE_ADDR'] ?? '0.0.0.0');
// 	// ONLY IPv4 is accepted in allow/deny list, so that we don't need GMP and IPv6-IPv4 mapping
// 	if(!empty($ip) || (strlen($ip) !== 4)) {
// 		$client = unpack('N', $ip)[1];
// 		foreach($allowNet as [$host, $cidr]) {
// 			$mask = ~((1 << (32 - $cidr)) - 1);
// 			if(($client & $mask) === ($host & $mask)) goto allowed;
// 		}
// 		goto denied;
// 	} else {
// 	denied:
// 		http_response_code(403);
// 		echo '<div style="text-align: center;"><h1>403 IP not allowed</h1><hr><small>SohaHongbao/2.024</small></div>';
// 		exit(0);
// 	}
// 	allowed: /* nothing to do */
// }
?><!DOCTYPE html>
<html lang="zh">
<head>
	<meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
	<title>新闻阅读 - foobar 院新年红包研究所</title>
	<style>
		html, body { margin: 0; padding: 0; font-size: 16px; }
		body { width: 100%; box-sizing: border-box; }
		a { color: blue; text-decoration: none; }
		a:hover { text-decoration: underline; }
		p, li { margin: 0.3em 0; line-height: 1.5; }
		#container {
			-moz-box-sizing: border-box;
			box-sizing: border-box;
			width: 100%;
			max-width: 800px;
			margin: 0 auto;
			padding-top: 20px;
		}
		#header {
			text-align: center;
			width: 100%;
			-moz-box-sizing: border-box;
			box-sizing: border-box;
			border: 1px #000 solid;
			-moz-box-shadow: 3px 5px 5px #888;
			box-shadow: 3px 5px 5px #888;
			padding: 15px 20px;
		}
		#header > span { margin: 4px 0; display: inline-block; }
		#title { font-size: 2em; text-shadow: 6px 5px 6px #133337; }
		#subtitle { font-size: 1.25em; font-style: italic; }
		#footer {
			font-size: .75em;
			text-align: center;
			width: 100%;
			-moz-box-sizing: border-box;
			box-sizing: border-box;
			padding: 4px 0;
		}
		#body-content { box-sizing: border-box; width: 100%; min-height: 260px; }
		#body-content > .big-one {
			margin-top: 24px;
			width: 100%;
			-moz-box-sizing: border-box;
			box-sizing: border-box;
			border: 1px #777 dashed;
			min-height: 260px;
			padding: 1em 2em;
			-moz-box-shadow: 0 0 3px #888;
			box-shadow: 0 0 3px #888;
			line-height: 1.25;
		}
		.big-one > .box-title {
			text-align: center;
			margin-top: 0;
			font-size: 1.325em;
			font-weight: bold;
		}
		.big-one p {
			margin: 1em 0;
		}
	</style>
</head>
<body>
	<div id="container">
		<div id="header">
			<span id="title">foobar 院新年<span style="color: red;">红包</span>研究所</span>
			<br />
			<span id="subtitle">新闻中心</span>
		</div>
		<div id="body-content">
			<div class="big-one">
				<p class="box-title"><?php echo $news['title']; ?></p>
				<hr />
				<?php echo $news['content']; ?>
			</div>
			<div style="font-size: .75em; text-align: right;">
				<a href="index.php">返回首页</a>
			</div>
		</div>
		<div id="footer">
			<p><a href="https://foobar.ac.cn">foobar 院</a> 信息技术中心&copy; 2024</p>
		</div>
	</div>
</body>
</html>