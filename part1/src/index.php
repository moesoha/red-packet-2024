<?php
include_once './news.inc.php';
?><!DOCTYPE html>
<html lang="zh">
	<head>
		<meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
		<title>foobar 院新年红包研究所</title>
		<style>
			html, body {
				margin: 0;
				padding: 0;
				font-size: 16px;
				font-family: Georgia, -apple-system, 'Nimbus Roman No9 L', 'PingFang SC', 'Hiragino Sans GB', 'Noto Sans CJK SC', 'Source Han Sans SC', 'Microsoft YaHei', 'WenQuanYi Micro Hei', 'ST Heiti', serif;
			}

			a {
				color: #148fd7;
				text-decoration: none;
			}

			a:hover {
				text-decoration: underline;
			}

			#container {
				min-height: 100vh;
				width: 100vw;
				-moz-box-sizing: border-box;
				box-sizing: border-box;
				position: relative;
			}

			#container > div {
				background-color: #eee;
				width: 100%;
				height: 300px;
				margin: 0;
				position: absolute;
				top: 50%;
				-moz-transform: translateY(-50%);
				-ms-transform: translateY(-50%);
				transform: translateY(-50%);
			}

			#container > div::after {
				display: block;
				clear: both;
				content: '';
			}

			.lpane, .rpane {
				-moz-box-sizing: border-box;
				box-sizing: border-box;
				height: 300px;
				padding: 10px 20px;
			}

			.lpane {
				width: 60%;
				float: left;
				text-align: right;
			}

			.rpane {
				width: 40%;
				background-color: #ddd;
				float: right;
				text-align: left;
			}

			.random-color {
				transition: all .3s ease;
			}
		</style>
	</head>
	<body>
		<div id="container">
			<div>
				<div class="lpane">
					<h1 style="margin: 0;">foobar 院新年<span style="color: red;">红包</span>研究所</h1>
					<h2 style="margin-top: 0;">所内综合办公系统</h2>
					<div style="display: inline-block; max-width: 380px;">
						<marquee class="random-color">
							<span>祝大家龙年快乐！新春愉快！寒假开心！</span>
						</marquee>
						<h3 style="text-align: left; margin-bottom: 0;">新闻</h3>
						<ol style="text-align: left; margin: 0; font-size: .875em;">
							<?php foreach(NEWS as $id => $data): if(($data['hidden'] ?? false) || ($data['internal'] ?? false)) continue; ?>
							<li><a href="news.php?id=<?php echo $id; ?>"><?php echo $data['title']; ?></a></li>
							<?php endforeach; ?>
						</ol>
					</div>
				</div>
				<div class="rpane">
					<br/>
					<br/>
					<br/>
					<div>
						<label for="username">用户名</label>
						<input type="text" name="username" />
					</div>
					<br/>
					<div>
						<label for="password">密码</label>
						<input type="password" name="password" />
					</div>
					<br/>
					<div>
						<button onclick="alert('系统维护中，暂不能登录。'/* 有没有一种可能，这真的只是个摆设 */);">登录</button>
					</div>
				</div>
			</div>
		</div>
		<script>
			setTimeout(function () {
				var l1 = '6789ab'.split(''), l2 = '23456789abcd'.split('');
				var l1l = l1.length, l2l = l2.length;
				var randColor = function () {
					var color = '#';
					for (var i = 0; i < 3; i++) {
						color += l1[Math.floor(Math.random() * l1l)];
						color += l2[Math.floor(Math.random() * l2l)];
					}
					return color;
				};
				var $s = document.getElementsByClassName('random-color');
				for (var i = 0; i < $s.length; i++) {
					var $ = $s[i];
					setInterval(function () {
						$.style.color = randColor();
					}, 300);
				}
			}, 0);
		</script>
	</body>
</html>
