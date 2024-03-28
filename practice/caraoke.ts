function main(lines: string[]) {
    let totalCharge = 0;
    let enterTime = '';
    let drinkCourse = '';
    let totalPeopleEntered = 0;
    let totalPeopleLeft = 0;
    let totalDrinksOrdered = 0;
    const rate = {
      one_drink: { day: 100, night: 400 },
      free_refills: { day: 200, night: 500 },
      alcohol_free_refills: { day: 300, night: 650 },
    };
  
    // 時刻を分単位に変換
    function convertTimeToMinutes(time: string): number {
      const [hours, minutes, seconds] = time.split(':').map(Number);
      return hours * 60 + minutes + (seconds > 0 ? 1 : 0);
    }
  
    // ナイトタイム開始時刻を計算
    function calculateNightTimeStart(enterTime: string): number {
      const [hours, ,] = enterTime.split(':').map(Number);
      return hours < 17 ? convertTimeToMinutes("18:50:00") : convertTimeToMinutes("17:50:00");
    }
  
    // 料金計算
    function calculateCharge(enterTime: string, leaveTime: string, course: string) {
      let charge = 0;
      const startMinutes = convertTimeToMinutes(enterTime);
      const endMinutes = convertTimeToMinutes(leaveTime);
      const nightStart = calculateNightTimeStart(enterTime);
      let currentTime = startMinutes;
  
      while (currentTime < endMinutes) {
        let isDaytime = currentTime < nightStart;
        let pricePer30Min = isDaytime ? rate[course].day : rate[course].night;
        charge += pricePer30Min;
        currentTime += 30; // 次の30分単位へ
      }
      return charge;
    }
  
    try {
      lines.forEach((line) => {
        const [time, recordType, ...args] = line.split(' ');
        switch (recordType) {
          case 'header':
            drinkCourse = args[0];
            break;
          case 'enter':
            const peopleEntered = parseInt(args[0], 10);
            if (peopleEntered >= 1000) throw { code: 99 };
            totalPeopleEntered += peopleEntered;
            if (!enterTime) enterTime = time; // 入室時刻を設定
            break;
          case 'leave':
            const peopleLeft = parseInt(args[0], 10);
            totalPeopleLeft += peopleLeft;
            if (totalPeopleLeft > totalPeopleEntered) throw { code: 99 };
            totalCharge += calculateCharge(enterTime, time, drinkCourse);
            enterTime = ''; // 退室時に入室時刻をリセット
            break;
          case 'drink':
            const [price, quantity] = args.map(Number);
            totalDrinksOrdered += quantity;
            totalCharge += price * quantity;
            break;
          case 'food':
            const [foodPrice, foodQuantity] = args.map(Number);
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
    } catch (err) {
      console.log(JSON.stringify(err));
    }
  }
  
function readFromStdin(): Promise<string[]> {
    return new Promise(resolve => {
      let data: string = "";
      process.stdin.resume();
      process.stdin.setEncoding("utf8");
      process.stdin.on("data", d => {
        data += d;
      });
      process.stdin.on("end", () => {
        resolve(data.split("\n").filter(line => line.trim())); // 空行を除外
      });
    })
}
  
  