<?php
function redistribute($a) {
    $max = -1;
    $max_index = 0;
    foreach ($a as $index => $block) {
        if ($block > $max) {
            $max = $block;
            $max_index = $index;
        }
    }
    $to_distribute = $a[$max_index];
    $a[$max_index] = 0;
    $di = ($max_index + 1) % count($a);
    while ($to_distribute > 0) {
        $a[$di] += 1;
        $di = ($di + 1) % count($a);
        $to_distribute -= 1;
    }
    return $a;
}

$f = fopen('input', 'r');
$line = fgets($f, 1024);
$blocks = array_filter(explode("\t", $line), function($num) {
    return strlen($num) > 0;
});
$blocks = array_map(function($x) {
    return intval($x);
}, $blocks);

$configurations = array();
$configurations[implode('_', $blocks)] = TRUE;
$cycle = 0;
while (true) {
    $cycle += 1;
    $blocks = redistribute($blocks);
    $key = implode('_', $blocks);
    if ($configurations[$key]) {
        break;
    }
    $configurations[$key] = TRUE;
}

print('cycles: ' . $cycle . "\n");
?>