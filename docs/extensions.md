# 可选排盘注记：规则、适用范围与核验

核阅日期：2026-09-19。基础盘约定见 [algorithm-sources.md](algorithm-sources.md)。本页定义暗干、旺衰、十二长生、刑墓、门迫和日马的计算口径，不提供吉凶断语。

## “常用”不等于所有流派唯一标准

这些概念大多有传统文献依据，也广泛用于时家拆补转盘的读盘；它们不都是组成基本盘的必备盘层，更不存在一套跨所有奇门流派、所有软件的唯一注记规范。应将其作为可选扩展，并在结果中记录启用项和规则。

| 注记 | 性质 | 与当前盘式的关系 | 向其他盘式复用的条件 |
| --- | --- | --- | --- |
| 暗干 | 另加的干盘，起法有分歧 | 本轮支持明确命名的值使起时干法；门下藏干仅作流派对照 | 必须核对该盘式的值使、旬首、宫序及中宫约定，不能无条件套用 |
| 旺衰 | 五行关系注记，九星另有传统规则 | 分别评价落宫和月令；星与门、干不能共用同一张状态表 | 五行关系函数可复用，但主柱、星门体系和月令口径须确认 |
| 十二长生 | 十干与十二地支的循环关系 | 对每个实际干及落宫地支分别计算，保留寄干身份 | 干支循环可复用；宫支、土的起点和阴阳顺逆约定须明确 |
| 六仪击刑 | 六甲所遁六仪与地支刑的传统规则 | 逐干标注，区分天盘、地盘及寄干 | 有相同六仪与宫支含义时可复用；不等同于任意两干相刑 |
| 入墓 | 至少包含干支长生墓、古典三奇入墓、时干入墓等不同问题 | 十干十二长生墓与古典三奇墓分别可选，见下文区别 | 不得将不同含义合并成一个无来源的布尔值 |
| 门迫 | 门五行克所在宫五行 | 依现有八门落宫计算；宫克门另称受制，本轮未输出该项 | 使用同样门宫五行关系时可复用 |
| 日马 | 按日支求驿马 | 与已有时马同时存在，不改变时马或时空显示依据 | 按任意指定柱地支求马的底层映射可复用 |

拆补、置闰主要解决定局问题；上述注记多以已经算好的盘与历法结果为输入，所以不应再各自计算一套节气或四柱。未来实现其他盘式时，仍需为其增加独立参考例，不能因为注记函数可复用就宣称已经支持那种盘式。

## 接口与架构约定

扩展配置属于 `qimen-core`；`qimen-calendar` 不依赖这些术数注记。计算过程保持确定性，不读取当前时间、环境变量或网络。

- 默认关闭全部扩展；旧调用的基础盘结果保持其原有计算口径。
- 调用者可以在库计算器初始化时选择扩展，也可以在单次计算参数中明确选择。
- CLI 通过选项启用，MCP 通过工具调用参数启用；语言绑定传入相同配置，不实现第二套算法。
- 输出保留所用规则、宫号、干所在盘层、源宫与寄宫身份；同一宫的两个干、两个地支不能压缩成无法对应的字符串。
- 天地盘干都参与已开启的逐干注记；若同时开启暗干，构造出的暗干也参与，并以独立 `hidden` 盘层标明。这是明确的派生计算范围，不表示各流派都同样使用暗干的刑墓断法。
- “未开启”“所选规则不适用此干”“已开启且未命中”是不同含义。中宫没有固定地支，长生分支为空，也不会命中某一墓支；八门在中宫不存在，不能伪造门迫。

具体参数名、可接受的枚举值与 JSON 结构以生成的 [请求 schema](schema/request.schema.json)、[结果 schema](schema/chart.schema.json) 和公共 Rust 类型为准。

Rust 可通过 `Calculator::new(ExtensionOptions::all())` 一次设置，随后多次调用 `calculate(&ChartRequest)`；只开启某项时，在 `ExtensionOptions` 中给该项设置明确的规则枚举。单次计算使用 `calculate_with_options(&request, &options)`。原 `calculate(&request)` 仍全部关闭。

CLI 示例：

```sh
qimen paipan --year 2026 --month 9 --day 18 --hour 18 --minute 15 --extensions all
qimen paipan --year 2026 --month 9 --day 18 --hour 18 --extensions hidden-stems,day-horse --json
qimen paipan --year 2026 --month 9 --day 18 --hour 18 --extensions tombs --tomb-rule traditional-three-wonders
```

JSON、MCP 工具参数使用同一 `CalculationRequest`：

```json
{
  "year": 2026,
  "month": 9,
  "day": 18,
  "hour": 18,
  "minute": 15,
  "utc_offset_minutes": 480,
  "extensions": {
    "hidden_stems": "duty_door_hour_stem_with_center_fallback",
    "day_horse": "day_branch_three_harmony"
  }
}
```

JSON 中省略的扩展不计算；未知选项或规则必须报错，不能悄悄降级。`--tomb-rule` 要与 `--extensions tombs` 或 `all` 同用，不能设置了规则却没有实际开启计算。

## 暗干

### 值使起时干，重干起中宫

固定三奇六仪顺序 `S = [戊, 己, 庚, 辛, 壬, 癸, 丁, 丙, 乙]`。取实际时干 `h`；甲时以该时旬首所遁六仪代甲。令 `k` 为 `h` 在 `S` 的索引。

1. 以值使门实际落宫为起宫 `p`，使用本项目已经完成中宫寄宫处理的结果。
2. 如果 `h` 与 `p` 宫的**本位地盘干**相同，则改以中五为起宫。此项是本项目明确选择的重干规则；不能只用“值符、值使同宫”代替，因为寄宫干与本位地盘干并非同一字段。
3. 阳遁取 `d = +1`，阴遁取 `d = -1`。对 `i = 0..8`，把 `S[(k + i) mod 9]` 放入 `1 + rem_euclid(p - 1 + d * i, 9)` 宫。

这里走的是数字九宫，包含中五，不是转盘八宫外环。暗干中五干也不自动复制到坤二。甲时没有可见“甲”干，必须先完成遁甲替代，再做重干检查。

该分支及重干条件可直接核对维护者的 [qimen-go 实现](https://github.com/deminzhang/qimen-go/blob/4d3f58fa0f401b5b3a337f119138e99e90685dda/xuan/qimen.go#L318-L336) 和 [暗干分支](https://github.com/deminzhang/qimen-go/blob/4d3f58fa0f401b5b3a337f119138e99e90685dda/xuan/qimen.go#L517-L538)，选项含义见其 [规则定义](https://github.com/deminzhang/qimen-go/blob/4d3f58fa0f401b5b3a337f119138e99e90685dda/xuan/qimen_defs.go#L79-L85)。这是可复核的现代排盘约定，不据此断言古籍规定所有暗干只能这样排。

### 流派对照：门下藏干（本轮未实现）

各宫当前八门携带其本位宫原有的地盘干。例如当前开门在哪一宫，该宫门下藏干就取地盘乾六的干。门本位为：休一、生八、伤三、杜四、景九、死二、惊七、开六。中五没有八门，因此没有这一法的暗干；不把中宫寄干追加为第二个暗干。

它与上一种九宫飞布法不等价，不应在对盘时静默切换。该法同样见上述 qimen-go 的 `QMHideGanDoorHomeGan` 分支。

## 旺衰：必须区分对象与参照

参照环境有两种：当前落宫五行，以及八字**节气月柱月支**的五行。辰、戌、丑、未月按土处理；不能用农历月份、固定公历月或者“秋天三个月全部属金”替换月支。

五行相生为木→火→土→金→水→木，相克为木→土→水→火→金→木。下表的“对象”是被评价的星、门或干，“环境”是宫或月支：

| 关系 | 九星状态（烟波法） | 八门、天干状态（一般五行法） |
| --- | --- | --- |
| 对象与环境同五行 | 相 | 旺 |
| 对象生环境 | 旺 | 休 |
| 环境生对象 | 废 | 相 |
| 对象克环境 | 休 | 囚 |
| 环境克对象 | 囚 | 死 |

九星五行由原宫决定：蓬水，冲辅木，英火，芮禽任土，心柱金。八门五行为休水，伤杜木，景火，生死土，开惊金。天干五行为甲乙木、丙丁火、戊己土、庚辛金、壬癸水。

[《烟波钓叟歌》](https://zh.wikisource.org/zh-hant/煙波釣叟歌)明确给出九星的特殊旺相关系；一般五行旺衰与十干生死可核对[《三命通会》卷二](https://zh.wikisource.org/wiki/三命通會/卷二)。本项目把同一关系规则分别应用于宫、月两种参照，并在输出中分开表示。对星、门或干的五行关系注记不等于综合强弱评分，也不等于吉凶结论。

## 十二长生：阳顺阴逆、土随火

固定阶段顺序，长生索引为零：

`长生、沐浴、冠带、临官、帝旺、衰、病、死、墓、绝、胎、养`

| 干 | 长生起支 | 方向 | 十二长生墓支 |
| --- | --- | --- | --- |
| 甲 | 亥 | 顺 | 未 |
| 乙 | 午 | 逆 | 戌 |
| 丙 | 寅 | 顺 | 戌 |
| 丁 | 酉 | 逆 | 丑 |
| 戊 | 寅 | 顺 | 戌 |
| 己 | 酉 | 逆 | 丑 |
| 庚 | 巳 | 顺 | 丑 |
| 辛 | 子 | 逆 | 辰 |
| 壬 | 申 | 顺 | 辰 |
| 癸 | 卯 | 逆 | 未 |

令支按子到亥编号 `0..11`，长生起支编号为 `s`，待查支为 `b`：阳干阶段索引为 `rem_euclid(b-s, 12)`，阴干为 `rem_euclid(s-b, 12)`。戊随丙、己随丁即本轮明确采用的“土随火”，依据见[《三命通会》卷二“论天干阴阳生死”](https://zh.wikisource.org/wiki/三命通會/卷二#论天干阴阳生死)。不把其他术数中的水土同生、五行同生同死等约定混入这个算法。

宫支映射为：坎子、坤未申、震卯、巽辰巳、乾戌亥、兑酉、艮丑寅、离午，中五无支。对一宫中的每个干分别返回各支阶段。例如丙在乾宫返回“戌：墓，亥：绝”，不是只返回一个“墓”；寄壬在乾宫另返回“戌：冠带，亥：临官”。

## 刑、墓与门迫

### 六仪击刑

| 可见六仪 | 所遁六甲 | 命中地支 | 宫 |
| --- | --- | --- | ---: |
| 戊 | 甲子 | 卯 | 3 |
| 己 | 甲戌 | 未 | 2 |
| 庚 | 甲申 | 寅 | 8 |
| 辛 | 甲午 | 午 | 9 |
| 壬 | 甲辰 | 辰 | 4 |
| 癸 | 甲寅 | 巳 | 4 |

依据为[《遁甲演义》“六仪击刑”](https://zh.wikisource.org/wiki/遁甲演義)：六甲所遁仪临其相刑或自刑之地。只依表检查，不将所有六仪都按其表面五行去推“刑”，也不将三奇受制混叫六仪击刑。

传统叙述着重于天盘六仪落宫。本项目另外对地盘及寄干按相同位置关系给出注记，若已开启暗干也包含该盘层；各盘层身份必须保留。这使软件能展示用户截图中地盘癸落巽宫的“刑”，而不会误报为天盘癸击刑，也不会把暗干注记混成天盘判断。

### 当前支持的入墓范围

开启刑墓后默认使用 `GrowthStageFireEarth`，即上表十干十二长生的墓支：该干所在宫包含其墓支，即命中。也可显式选择 `TraditionalThreeWonders`，只检查古典三奇乙未、丙戌、丁丑；这一选项对六仪不适用，墓支返回空值，不能把它理解成六仪永不入墓。两种入墓规则均与是否启用十二长生的完整展示独立；单开刑墓也能计算墓支。天盘、地盘、寄干分别判断。

不要混淆三种问题：

- **盘干临墓**：某个实际干落入包含其墓支的宫，是当前输出的逐干注记。
- **时干自身坐墓**：例如丙戌时、壬辰时，检查的是时柱干支自身；它不是“找到任何一个墓宫就称时干入墓”。《遁甲演义》另有相关段落。本轮不单列这个全局格局。
- **古典三奇入墓**：《遁甲演义》明确列乙临坤未、丙临乾戌、丁临艮丑；其中乙未和阴干十二长生的乙戌不同。`TraditionalThreeWonders` 明确选取这组三奇规则，不把乙未追加进默认十干墓表，也不宣称实现了所有三奇入墓版本。

例如选择古典三奇墓并同时开启十二长生时，乙在坤二可显示“未：养”及“古典三奇墓支：未”；两者回答的是不同规则下的问题。这不是算法矛盾，消费者应展示规则名，不能把长生的“养”改成“墓”来追求文字一致。

日干和时干相同、或寄干重复显示时，不能丢失对应关系；基础四柱和干的盘层身份可以供消费者准确识别所需对象。

### 门迫与受制

门克宫为门迫；宫克门为受制。这两个方向必须分别辨别，不能互换；当前门迫字段只在门克宫时命中。[《遁甲演义》“门迫宫迫”](https://zh.wikisource.org/wiki/遁甲演義)提供了具体例子：开门落震三、休门落离九、生门落坎一、景门落兑七等。

例如开门金落震三木为门迫；开门金落离九火为受制。门生宫、宫生门、同五行均不因此产生门迫。中心没有门，不存在独立门迫。

## 日马

| 被查询柱的地支集合 | 驿马支 | 宫 |
| --- | --- | ---: |
| 申、子、辰 | 寅 | 8 |
| 寅、午、戌 | 申 | 2 |
| 亥、卯、未 | 巳 | 4 |
| 巳、酉、丑 | 亥 | 6 |

该三合驿马映射在[《三命通会》卷三“论驿马”](https://zh.wikisource.org/wiki/三命通會/卷三#论驿马)有逐组说明。日马取已经按调用者日界规则算出的日支，时马取时支；不能为了计算日马重新按另一种换日规则取日期。开启日马不更改基础盘中按时支标记的马星。

## 用户截图核验例

来源：用户于 2026-09-18 提供的软件截图，画面时间为北京时间 `2026-09-18 18:15:00`，四柱丙午、丁酉、乙未、乙酉，时家拆补转盘、阴遁九局、甲申旬。截图用于核对可见字段，不能证明该软件所有未展示分支的取法。

值使惊门落乾六，时干乙，乾六本位地盘辛，未触发重干起中宫。值使起时干法的各宫暗干应为：

| 宫 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 暗干 | 壬 | 辛 | 庚 | 己 | 戊 | 乙 | 丙 | 丁 | 癸 |

星门旺衰应为下表；斜线前是落宫状态，后是酉月状态：

| 宫 | 星 | 星：宫／月 | 门 | 门：宫／月 |
| --- | --- | --- | --- | --- |
| 1 | 天柱 | 旺／相 | 开 | 休／旺 |
| 2 | 天辅 | 休／囚 | 景 | 休／囚 |
| 3 | 天蓬 | 旺／废 | 生 | 死／休 |
| 4 | 天任 | 囚／旺 | 伤 | 旺／死 |
| 6 | 天芮、天禽 | 各旺／旺 | 惊 | 旺／旺 |
| 7 | 天英 | 休／休 | 死 | 休／休 |
| 8 | 天心 | 废／相 | 休 | 死／相 |
| 9 | 天冲 | 旺／囚 | 杜 | 休／死 |

其他可见固定预期：

- 日支未，日马巳在巽四；时支酉，时马亥在乾六，两者不同。
- 天盘癸在坤二：未为墓，申为死；地盘丙在同宫：未为衰，申为病。
- 天盘丙在乾六：戌为墓，亥为绝；同宫天盘寄壬：戌为冠带，亥为临官。
- 地盘己在艮八：丑为墓，寅为死；地盘癸在巽四：辰为养，巳为胎，并在巳支命中六仪击刑。
- 该盘没有门迫；震三生门、艮八休门属于宫克门的受制关系。

截图中的数字串、宫位流转箭头、月建之外的综合判语等未在本页定义，不能凭外观猜测其含义，也不纳入扩展核验通过范围。

## 独立边界用例

除了截图，回归测试应能区分以下容易混淆的实现：

| 用例 | 固定预期与目的 |
| --- | --- |
| 阳一局甲子时，值使在 1 | 甲以戊代，地盘 1 为戊，重干起中 5；暗干 1..9 为癸丁丙乙戊己庚辛壬 |
| 阳一局乙丑时，值使在 2 | 乙与本位己不同，起 2；暗干 1..9 为丙乙戊己庚辛壬癸丁 |
| 阴九局乙酉时 | 采用用户截图的九宫暗干预期，检验阴遁逆飞与循环回绕 |
| 值使寄宫，时干等于中宫寄干但不同于本位干 | 不因寄干相等而误触发本位重干规则 |
| 十二长生：壬申、壬子、壬辰 | 分别长生、帝旺、墓，可发现抄表偏移 |
| 十二长生：丁酉、丁丑；戊寅、己酉 | 分别长生、墓；戊己均为长生，验证阴逆与土随火 |
| 乙未与乙戌 | 十二长生分别养、墓；默认十干墓取戌，显式古典三奇墓取未，两种规则不可混并 |
| 古典三奇墓检查六仪 | 墓支为空值，表示此规则不适用；六仪击刑仍照其独立规则计算 |
| 六仪击刑六个真值及同干相邻宫 | 正反例配对，验证完整表而非仅当前盘的一项 |
| 节气月界与农历月界 | 月令跟随 calendar 月柱；不随农历初一错误切换 |
| 子初与午夜换日配置 | 日马跟随日柱的实际换日；时马仍依据时柱 |
| 单开、全开、全关 | 验证独立开关、默认兼容性及无依赖字段缺失 |

## 参考实现与许可

本页引用原典规则并自行整理公式，不复制现代注解的断语或上游生产代码。

- `deminzhang/qimen-go`，提交 `4d3f58fa0f401b5b3a337f119138e99e90685dda`：[MIT，Copyright 2024 deminzhang](https://github.com/deminzhang/qimen-go/blob/4d3f58fa0f401b5b3a337f119138e99e90685dda/LICENSE)。核阅暗干的两个独立起法、遁甲替代与重干分支，不照抄其其他表格。
- `3metaJun/3meta`，提交 `9be1238cbb7b0118826a689f9d3f8100284f6df3`：[MIT，Copyright 2025 3metaJun](https://github.com/3metaJun/3meta/blob/9be1238cbb7b0118826a689f9d3f8100284f6df3/LICENSE)。[暗干实现](https://github.com/3metaJun/3meta/blob/9be1238cbb7b0118826a689f9d3f8100284f6df3/src/qimen/calculator.ts)另以甲时、中宫干、值符值使同宫作分支；不能假设它与本项目重干法在所有寄宫案例都一致。其[常量](https://github.com/3metaJun/3meta/blob/9be1238cbb7b0118826a689f9d3f8100284f6df3/src/data/constants.ts)把亥卯未驿马列为申，与上述原典和用户截图的巳不符；其[长生表](https://github.com/3metaJun/3meta/blob/9be1238cbb7b0118826a689f9d3f8100284f6df3/src/analysis/index.ts)中壬的多个支也与所选阳顺规则不符。因此只作差异研究，不把该软件的完整输出当作正确答案。

## English summary

These annotations are common traditional concepts, not a single universal specification shared by every Qimen school. They are optional `qimen-core` calculations derived from the existing chart and calendar result. Defaults remain off; every enabled result identifies its rule, palace, plate and hosted-stem provenance. CLI, MCP and language bindings expose the same configuration without duplicating algorithms.

The implemented hidden-stem convention flies the effective hour stem from the duty-door palace in numeric nine-palace order, moving the start to palace five when it equals the native earth stem. A Jia hour uses its xun-hidden instrument first. Carrying each door's original earth stem with that door is a distinct convention documented for comparison, not implemented in this release. These methods must not be silently interchanged.

Star strength follows the Yanbo convention; door and stem strength use ordinary Five-Phase relations. Palace and solar-month contexts are reported separately. The twelve growth stages use yang-forward/yin-backward traversal, with Wu following Bing and Ji following Ding. Tomb annotations default to that same ten-stem rule. An explicit `TraditionalThreeWonders` option instead uses Yi at Wei, Bing at Xu and Ding at Chou; other stems return no applicable tomb branch under that rule. It is never silently combined with Yi's growth-stage tomb at Xu, and does not alter the separate growth-stage calculation. Instrument punishment and door pressure retain their exact direction and plate identity. The day horse uses the computed day branch, while the existing hour horse continues to use the hour branch.

The screenshot fixture is `2026-09-18 18:15:00 +08:00`. It verifies the visible hidden stems, star/door strength, selected growth/tomb/punishment entries and the distinct day/hour horses. It does not validate every school convention or proprietary annotation shown by that application. Future chart methods require their own fixtures before support is claimed.
