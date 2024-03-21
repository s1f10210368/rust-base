function main(lines) {
    var totalCharge = 0;
    var enterTime = '';
    var drinkCourse = '';
    var totalPeopleEntered = 0;
    var totalPeopleLeft = 0;
    var totalDrinksOrdered = 0;
    var rate = {
        one_drink: { day: 100, night: 400 },
        free_refills: { day: 200, night: 500 },
        alcohol_free_refills: { day: 300, night: 650 },
    };
    // 時刻を分単位に変換
    function convertTimeToMinutes(time) {
        var _a = time.split(':').map(Number), hours = _a[0], minutes = _a[1], seconds = _a[2];
        return hours * 60 + minutes + (seconds > 0 ? 1 : 0);
    }
    // ナイトタイム開始時刻を計算
    function calculateNightTimeStart(enterTime) {
        var _a = enterTime.split(':').map(Number), hours = _a[0];
        return hours < 17 ? convertTimeToMinutes("18:50:00") : convertTimeToMinutes("17:50:00");
    }
    // 料金計算
    function calculateCharge(enterTime, leaveTime, course) {
        var charge = 0;
        var startMinutes = convertTimeToMinutes(enterTime);
        var endMinutes = convertTimeToMinutes(leaveTime);
        var nightStart = calculateNightTimeStart(enterTime);
        var currentTime = startMinutes;
        while (currentTime < endMinutes) {
            var isDaytime = currentTime < nightStart;
            var pricePer30Min = isDaytime ? rate[course].day : rate[course].night;
            charge += pricePer30Min;
            currentTime += 30; // 次の30分単位へ
        }
        return charge;
    }
    try {
        lines.forEach(function (line) {
            var _a = line.split(' '), time = _a[0], recordType = _a[1], args = _a.slice(2);
            switch (recordType) {
                case 'header':
                    drinkCourse = args[0];
                    break;
                case 'enter':
                    var peopleEntered = parseInt(args[0], 10);
                    if (peopleEntered >= 1000)
                        throw { code: 99 };
                    totalPeopleEntered += peopleEntered;
                    if (!enterTime)
                        enterTime = time; // 入室時刻を設定
                    break;
                case 'leave':
                    var peopleLeft = parseInt(args[0], 10);
                    totalPeopleLeft += peopleLeft;
                    if (totalPeopleLeft > totalPeopleEntered)
                        throw { code: 99 };
                    totalCharge += calculateCharge(enterTime, time, drinkCourse);
                    enterTime = ''; // 退室時に入室時刻をリセット
                    break;
                case 'drink':
                    var _b = args.map(Number), price = _b[0], quantity = _b[1];
                    totalDrinksOrdered += quantity;
                    totalCharge += price * quantity;
                    break;
                case 'food':
                    var _c = args.map(Number), foodPrice = _c[0], foodQuantity = _c[1];
                    totalCharge += foodPrice * foodQuantity;
                    break;
                case 'footer':
                    // ワンドリンクエラーのチェック
                    if (drinkCourse === 'one_drink' && totalDrinksOrdered < totalPeopleEntered) {
                        throw {
                            code: 1,
                            price: totalCharge,
                            drink: totalPeopleEntered - totalDrinksOrdered,
                        };
                    }
                    console.log(JSON.stringify({ code: 0, price: totalCharge }));
                    break;
            }
        });
    }
    catch (err) {
        console.log(JSON.stringify(err));
    }
}
function readFromStdin() {
    return new Promise(function (resolve) {
        var data = "";
        process.stdin.resume();
        process.stdin.setEncoding("utf8");
        process.stdin.on("data", function (d) {
            data += d;
        });
        process.stdin.on("end", function () {
            resolve(data.split("\n").filter(function (line) { return line.trim(); })); // 空行を除外
        });
    });
}
