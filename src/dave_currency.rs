use iso_currency::Currency;
use std::io;

pub struct ExchangeResult {
	original_currency: String,
	original_amount: f32,
	final_currency: String,
	final_amount: f32,
}

fn match_from_currency(from_currency: &str, to_currency: &str, num: &f32) -> f32 {
	match &*from_currency.trim() {
		"USD" => convert_from_usd(&to_currency, &num),
		"EUR" => convert_from_eur(&to_currency, &num),
		"GBP" => convert_from_gbp(&to_currency, &num),
		"CAD" => convert_from_cad(&to_currency, &num),
		"JPY" => convert_from_jpy(&to_currency, &num),
		"CNY" => convert_from_cny(&to_currency, &num),
		"AUD" => convert_from_aud(&to_currency, &num),
		"CHF" => convert_from_chf(&to_currency, &num),
		"SEK" => convert_from_sek(&to_currency, &num),
		"INR" => convert_from_inr(&to_currency, &num),
		"KRW" => convert_from_krw(&to_currency, &num),
		"NOK" => convert_from_nok(&to_currency, &num),
		"NZD" => convert_from_nzd(&to_currency, &num),
		"RUB" => convert_from_rub(&to_currency, &num),
		"BRL" => convert_from_brl(&to_currency, &num),
		"SAR" => convert_from_sar(&to_currency, &num),
		"ILS" => convert_from_ils(&to_currency, &num),
		"DKK" => convert_from_dkk(&to_currency, &num),
		"PLN" => convert_from_pln(&to_currency, &num),
		"MXN" => convert_from_mxn(&to_currency, &num),
		_ => 0.0,
	}
}

fn convert_from_usd(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_usd_rate = 0.78343;
	let eur_to_usd_rate = 0.91623;
	let jpy_to_usd_rate = 148.184;
	let cad_to_usd_rate = 1.34905;
	let cny_to_usd_rate = 7.19503;
	let aud_to_usd_rate = 1.5151;
	let chf_to_usd_rate = 0.87983;
	let sek_to_usd_rate = 10.2616;
	let inr_to_usd_rate = 82.7497;
	let krw_to_usd_rate = 1326.82;
	let nok_to_usd_rate = 10.4495;
	let nzd_to_usd_rate = 1.62381;
	let rub_to_usd_rate = 90.7754;
	let brl_to_usd_rate = 4.9548;
	let sar_to_usd_rate = 3.74787;
	let ils_to_usd_rate = 3.57002;
	let dkk_to_usd_rate = 6.81177;
	let pln_to_usd_rate = 3.93091;
	let mxn_to_usd_rate = 16.8419;

	let amount: f32 = match to_currency {
		"EUR" => *num * eur_to_usd_rate,
		"GBP" => *num * gbp_to_usd_rate,
		"CAD" => *num * cad_to_usd_rate,
		"JPY" => *num * jpy_to_usd_rate,
		"CNY" => *num * cny_to_usd_rate,
		"AUD" => *num * aud_to_usd_rate,
		"CHF" => *num * chf_to_usd_rate,
		"SEK" => *num * sek_to_usd_rate,
		"INR" => *num * inr_to_usd_rate,
		"KRW" => *num * krw_to_usd_rate,
		"NOK" => *num * nok_to_usd_rate,
		"NZD" => *num * nzd_to_usd_rate,
		"RUB" => *num * rub_to_usd_rate,
		"BRL" => *num * brl_to_usd_rate,
		"SAR" => *num * sar_to_usd_rate,
		"ILS" => *num * ils_to_usd_rate,
		"DKK" => *num * dkk_to_usd_rate,
		"PLN" => *num * pln_to_usd_rate,
		"MXN" => *num * mxn_to_usd_rate,
		"USD" => *num,
		_ => 0.0,
	};

	amount
}

fn convert_from_jpy(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_jpy_rate = 0.00529;
	let eur_to_jpy_rate = 0.00618;
	let cad_to_jpy_rate = 0.0091;
	let usd_to_jpy_rate = 0.00675;
	let cny_to_jpy_rate = 0.04855;
	let aud_to_jpy_rate = 0.01022;
	let chf_to_jpy_rate = 0.00594;
	let sek_to_jpy_rate = 0.06924;
	let inr_to_jpy_rate = 0.55836;
	let krw_to_jpy_rate = 8.95286;
	let nok_to_jpy_rate = 0.07051;
	let nzd_to_jpy_rate = 0.01096;
	let rub_to_jpy_rate = 0.61585;
	let brl_to_jpy_rate = 0.03361;
	let sar_to_jpy_rate = 0.02543;
	let ils_to_jpy_rate = 0.02422;
	let dkk_to_jpy_rate = 0.04621;
	let pln_to_jpy_rate = 0.02667;
	let mxn_to_jpy_rate = 0.11426;

	let amount: f32 = match to_currency {
		"USD" => *num * usd_to_jpy_rate,
		"EUR" => *num * eur_to_jpy_rate,
		"GBP" => *num * gbp_to_jpy_rate,
		"CAD" => *num * cad_to_jpy_rate,
		"CNY" => *num * cny_to_jpy_rate,
		"AUD" => *num * aud_to_jpy_rate,
		"CHF" => *num * chf_to_jpy_rate,
		"SEK" => *num * sek_to_jpy_rate,
		"INR" => *num * inr_to_jpy_rate,
		"KRW" => *num * krw_to_jpy_rate,
		"NOK" => *num * nok_to_jpy_rate,
		"NZD" => *num * nzd_to_jpy_rate,
		"RUB" => *num * rub_to_jpy_rate,
		"BRL" => *num * brl_to_jpy_rate,
		"SAR" => *num * sar_to_jpy_rate,
		"ILS" => *num * ils_to_jpy_rate,
		"DKK" => *num * dkk_to_jpy_rate,
		"PLN" => *num * pln_to_jpy_rate,
		"MXN" => *num * mxn_to_jpy_rate,
		"JPY" => *num,
		_ => 0.0,
	};

	amount
}

fn convert_from_eur(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_eur_rate = 0.85499;
    let usd_to_eur_rate = 1.09128;
    let jpy_to_eur_rate = 161.719;
    let cad_to_eur_rate = 1.47219;
    let cny_to_eur_rate = 7.85179;
    let aud_to_eur_rate = 1.65355;
    let chf_to_eur_rate = 0.9602;
    let sek_to_eur_rate = 11.1986;
	let inr_to_eur_rate = 90.3031;
	let krw_to_eur_rate = 1447.93;
	let nok_to_eur_rate = 11.4038;
	let nzd_to_eur_rate = 1.77203;
	let rub_to_eur_rate = 99.3225;
	let brl_to_eur_rate = 5.42133;
	let sar_to_eur_rate = 4.10075;
	let ils_to_eur_rate = 3.90616;
	let dkk_to_eur_rate = 7.45381;
	let pln_to_eur_rate = 4.30141;
	let mxn_to_eur_rate = 18.4277;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_eur_rate,
        "GBP" => *num * gbp_to_eur_rate,
        "CAD" => *num * cad_to_eur_rate,
        "JPY" => *num * jpy_to_eur_rate,
        "CNY" => *num * cny_to_eur_rate,
        "AUD" => *num * aud_to_eur_rate,
        "CHF" => *num * chf_to_eur_rate,
        "SEK" => *num * sek_to_eur_rate,
		"INR" => *num * inr_to_eur_rate,
		"KRW" => *num * krw_to_eur_rate,
		"NOK" => *num * nok_to_eur_rate,
		"NZD" => *num * nzd_to_eur_rate,
		"RUB" => *num * rub_to_eur_rate,
		"BRL" => *num * brl_to_eur_rate,
		"SAR" => *num * sar_to_eur_rate,
		"ILS" => *num * ils_to_eur_rate,
		"DKK" => *num * dkk_to_eur_rate,
		"PLN" => *num * pln_to_eur_rate,
		"MXN" => *num * mxn_to_eur_rate,
        "EUR" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_gbp(to_currency: &str, num: &f32) -> f32 {
    let jpy_to_gbp_rate = 189.129;
    let eur_to_gbp_rate = 1.16941;
    let cad_to_gbp_rate = 1.72174;
    let usd_to_gbp_rate = 1.27626;
    let cny_to_gdp_rate = 9.18275;
    let aud_to_gbp_rate = 1.93367;
    let chf_to_gbp_rate = 1.12298;
    let sek_to_gbp_rate = 13.0965;
	let inr_to_gbp_rate = 105.61;
	let krw_to_gbp_rate = 1693.37;
	let nok_to_gbp_rate = 13.3363;
	let nzd_to_gbp_rate = 2.07241;
	let rub_to_gbp_rate = 116.499;
	let brl_to_gbp_rate = 6.35887;
	let sar_to_gbp_rate = 4.80992;
	let ils_to_gbp_rate = 4.58167;
	let dkk_to_gbp_rate = 8.74205;
	let pln_to_gbp_rate = 5.04483;
	let mxn_to_gbp_rate = 21.6145;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_gbp_rate,
        "EUR" => *num * eur_to_gbp_rate,
        "CAD" => *num * cad_to_gbp_rate,
        "JPY" => *num * jpy_to_gbp_rate,
        "CNY" => *num * cny_to_gdp_rate,
        "AUD" => *num * aud_to_gbp_rate,
        "CHF" => *num * chf_to_gbp_rate,
        "SEK" => *num * sek_to_gbp_rate,
		"INR" => *num * inr_to_gbp_rate,
		"KRW" => *num * krw_to_gbp_rate,
		"NOK" => *num * nok_to_gbp_rate,
		"NZD" => *num * nzd_to_gbp_rate,
		"RUB" => *num * rub_to_gbp_rate,
		"BRL" => *num * brl_to_gbp_rate,
		"SAR" => *num * sar_to_gbp_rate,
		"ILS" => *num * ils_to_gbp_rate,
		"DKK" => *num * dkk_to_gbp_rate,
		"PLN" => *num * pln_to_gbp_rate,
		"MXN" => *num * mxn_to_gbp_rate,
        "GBP" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_cad(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_cad_rate = 0.58065;
    let eur_to_cad_rate = 0.67907;
    let jpy_to_cad_rate = 109.828;
    let usd_to_cad_rate = 0.74116;
    let cny_to_cad_rate = 5.33268;
    let aud_to_cad_rate = 1.12293;
    let chf_to_cad_rate = 0.6521;
    let sek_to_cad_rate = 7.60548;
	let inr_to_cad_rate = 61.3309;
	let krw_to_cad_rate = 983.387;
	let nok_to_cad_rate = 7.74479;
	let nzd_to_cad_rate = 1.2035;
	let rub_to_cad_rate = 67.4361;
	let brl_to_cad_rate = 3.68087;
	let sar_to_cad_rate = 2.78425;
	let ils_to_cad_rate = 2.65213;
	let dkk_to_cad_rate = 5.06039;
	let pln_to_cad_rate = 2.92023;
	let mxn_to_cad_rate = 12.5117;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_cad_rate,
        "EUR" => *num * eur_to_cad_rate,
        "GBP" => *num * gbp_to_cad_rate,
        "JPY" => *num * jpy_to_cad_rate,
        "CNY" => *num * cny_to_cad_rate,
        "AUD" => *num * aud_to_cad_rate,
        "CHF" => *num * chf_to_cad_rate,
        "SEK" => *num * sek_to_cad_rate,
		"INR" => *num * inr_to_cad_rate,
		"KRW" => *num * krw_to_cad_rate,
		"NOK" => *num * nok_to_cad_rate,
		"NZD" => *num * nzd_to_cad_rate,
		"RUB" => *num * rub_to_cad_rate,
		"BRL" => *num * brl_to_cad_rate,
		"SAR" => *num * sar_to_cad_rate,
		"ILS" => *num * ils_to_cad_rate,
		"DKK" => *num * dkk_to_cad_rate,
		"PLN" => *num * pln_to_cad_rate,
		"MXN" => *num * mxn_to_cad_rate,
        "CAD" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_cny(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_cny_rate = 0.10885;
    let eur_to_cny_rate = 0.12731;
    let jpy_to_cny_rate = 20.5896;
    let usd_to_cny_rate = 0.13895;
    let cad_to_cny_rate = 0.18745;
    let aud_to_cny_rate = 0.21052;
    let chf_to_cny_rate = 0.12225;
    let sek_to_cny_rate = 1.42581;
	let inr_to_cny_rate = 11.4978;
	let krw_to_cny_rate = 184.357;
	let nok_to_cny_rate = 1.45192;
	let nzd_to_cny_rate = 0.22562;
	let rub_to_cny_rate = 12.6249;
	let brl_to_cny_rate = 0.6891;
	let sar_to_cny_rate = 0.52125;
	let ils_to_cny_rate = 0.49651;
	let dkk_to_cny_rate = 0.94737;
	let pln_to_cny_rate = 0.5467;
	let mxn_to_cny_rate = 2.34235;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_cny_rate,
        "EUR" => *num * eur_to_cny_rate,
        "GBP" => *num * gbp_to_cny_rate,
        "JPY" => *num * jpy_to_cny_rate,
        "CAD" => *num * cad_to_cny_rate,
        "AUD" => *num * aud_to_cny_rate,
        "CHF" => *num * chf_to_cny_rate,
        "SEK" => *num * sek_to_cny_rate,
		"INR" => *num * inr_to_cny_rate,
		"KRW" => *num * krw_to_cny_rate,
		"NOK" => *num * nok_to_cny_rate,
		"NZD" => *num * nzd_to_cny_rate,
		"RUB" => *num * rub_to_cny_rate,
		"BRL" => *num * brl_to_cny_rate,
		"SAR" => *num * sar_to_cny_rate,
		"ILS" => *num * ils_to_cny_rate,
		"DKK" => *num * dkk_to_cny_rate,
		"PLN" => *num * pln_to_cny_rate,
		"MXN" => *num * mxn_to_cny_rate,
        "CNY" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_aud(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_aud_rate = 0.51698;
    let eur_to_aud_rate = 0.60467;
    let jpy_to_aud_rate = 97.7911;
    let usd_to_aud_rate = 0.6599;
    let cad_to_aud_rate = 0.89023;
    let cny_to_aud_rate = 4.74799;
    let chf_to_aud_rate = 0.5806;
    let sek_to_aud_rate = 6.7716;
	let inr_to_aud_rate = 54.6065;
	let krw_to_aud_rate = 875.567;
	let nok_to_aud_rate = 6.89564;
	let nzd_to_aud_rate = 1.07165;
	let rub_to_aud_rate = 60.1816;
	let brl_to_aud_rate = 3.2849;
	let sar_to_aud_rate = 2.48473;
	let ils_to_aud_rate = 2.36683;
	let dkk_to_aud_rate = 4.51602;
	let pln_to_aud_rate = 2.60608;
	let mxn_to_aud_rate = 11.1657;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_aud_rate,
        "EUR" => *num * eur_to_aud_rate,
        "GBP" => *num * gbp_to_aud_rate,
        "JPY" => *num * jpy_to_aud_rate,
        "CAD" => *num * cad_to_aud_rate,
        "CNY" => *num * cny_to_aud_rate,
        "CHF" => *num * chf_to_aud_rate,
        "SEK" => *num * sek_to_aud_rate,
		"INR" => *num * inr_to_aud_rate,
		"KRW" => *num * krw_to_aud_rate,
		"NOK" => *num * nok_to_aud_rate,
		"NZD" => *num * nzd_to_aud_rate,
		"RUB" => *num * rub_to_aud_rate,
		"BRL" => *num * brl_to_aud_rate,
		"SAR" => *num * sar_to_aud_rate,
		"ILS" => *num * ils_to_aud_rate,
		"DKK" => *num * dkk_to_aud_rate,
		"PLN" => *num * pln_to_aud_rate,
		"MXN" => *num * mxn_to_aud_rate,
        "AUD" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_chf(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_chf_rate = 0.89033;
	let eur_to_chf_rate = 1.04125;
	let jpy_to_chf_rate = 168.404;
	let usd_to_chf_rate = 1.13638;
	let cad_to_chf_rate = 1.53302;
	let aud_to_chf_rate = 1.72172;
	let cny_to_chf_rate = 8.17625;
	let sek_to_chf_rate = 11.661;
	let inr_to_chf_rate = 94.0347;
	let krw_to_chf_rate = 1507.76;
	let nok_to_chf_rate = 11.8746;
	let nzd_to_chf_rate = 1.84526;
	let rub_to_chf_rate = 103.507;
	let brl_to_chf_rate = 5.64971;
	let sar_to_chf_rate = 4.2735;
	let ils_to_chf_rate = 4.07071;
	let dkk_to_chf_rate = 7.76712;
	let pln_to_chf_rate = 4.48221;
	let mxn_to_chf_rate = 19.204;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_chf_rate,
        "EUR" => *num * eur_to_chf_rate,
        "GBP" => *num * gbp_to_chf_rate,
        "JPY" => *num * jpy_to_chf_rate,
        "CAD" => *num * cad_to_chf_rate,
        "CNY" => *num * cny_to_chf_rate,
        "AUD" => *num * aud_to_chf_rate,
        "SEK" => *num * sek_to_chf_rate,
		"INR" => *num * inr_to_chf_rate,
		"KRW" => *num * krw_to_chf_rate,
		"NOK" => *num * nok_to_chf_rate,
		"NZD" => *num * nzd_to_chf_rate,
		"RUB" => *num * rub_to_chf_rate,
		"BRL" => *num * brl_to_chf_rate,
		"SAR" => *num * sar_to_chf_rate,
		"ILS" => *num * ils_to_chf_rate,
		"DKK" => *num * dkk_to_chf_rate,
		"PLN" => *num * pln_to_chf_rate,
		"MXN" => *num * mxn_to_chf_rate,
        "CHF" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_sek(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_sek_rate = 0.07631;
	let eur_to_sek_rate = 0.08925;
	let jpy_to_sek_rate = 14.4338;
	let usd_to_sek_rate = 0.0974;
	let cad_to_sek_rate = 0.1314;
	let aud_to_sek_rate = 0.14758;
	let cny_to_sek_rate = 0.70083;
	let chf_to_sek_rate = 0.0857;
	let inr_to_sek_rate = 8.06022;
	let krw_to_sek_rate = 129.239;
	let nok_to_sek_rate = 1.01783;
	let nzd_to_sek_rate = 0.15817;
	let rub_to_sek_rate = 8.88339;
	let brl_to_sek_rate = 0.48488;
	let sar_to_sek_rate = 0.36677;
	let ils_to_sek_rate = 0.34937;
	let dkk_to_sek_rate = 0.66661;
	let pln_to_sek_rate = 0.38468;
	let mxn_to_sek_rate = 1.64817;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_sek_rate,
        "EUR" => *num * eur_to_sek_rate,
        "GBP" => *num * gbp_to_sek_rate,
        "JPY" => *num * jpy_to_sek_rate,
        "CAD" => *num * cad_to_sek_rate,
        "CNY" => *num * cny_to_sek_rate,
        "AUD" => *num * aud_to_sek_rate,
        "CHF" => *num * chf_to_sek_rate,
        "INR" => *num * inr_to_sek_rate,
        "KRW" => *num * krw_to_sek_rate,
		"NOK" => *num * nok_to_sek_rate,
		"NZD" => *num * nzd_to_sek_rate,
		"RUB" => *num * rub_to_sek_rate,
		"BRL" => *num * brl_to_sek_rate,
		"SAR" => *num * sar_to_sek_rate,
		"ILS" => *num * ils_to_sek_rate,
		"DKK" => *num * dkk_to_sek_rate,
		"PLN" => *num * pln_to_sek_rate,
		"MXN" => *num * mxn_to_sek_rate,
        "SEK" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_inr(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_inr_rate = 0.00947;
	let eur_to_inr_rate = 0.01107;
	let jpy_to_inr_rate = 1.79059;
	let usd_to_inr_rate = 0.01208;
	let cad_to_inr_rate = 0.0163;
	let aud_to_inr_rate = 0.01831;
	let cny_to_inr_rate = 0.08694;
	let chf_to_inr_rate = 0.01063;
	let sek_to_inr_rate = 0.124;
	let krw_to_inr_rate = 16.0327;
	let nok_to_inr_rate = 0.12627;
	let nzd_to_inr_rate = 0.01962;
	let rub_to_inr_rate = 1.09715;
	let brl_to_inr_rate = 0.05989;
	let sar_to_inr_rate = 0.0453;
	let ils_to_inr_rate = 0.04315;
	let dkk_to_inr_rate = 0.08233;
	let pln_to_inr_rate = 0.04751;
	let mxn_to_inr_rate = 0.20356;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_inr_rate,
        "EUR" => *num * eur_to_inr_rate,
        "GBP" => *num * gbp_to_inr_rate,
        "JPY" => *num * jpy_to_inr_rate,
        "CAD" => *num * cad_to_inr_rate,
        "CNY" => *num * cny_to_inr_rate,
        "AUD" => *num * aud_to_inr_rate,
        "CHF" => *num * chf_to_inr_rate,
        "SEK" => *num * sek_to_inr_rate,
        "KRW" => *num * krw_to_inr_rate,
		"NOK" => *num * nok_to_inr_rate,
		"NZD" => *num * nzd_to_inr_rate,
		"RUB" => *num * rub_to_inr_rate,
		"BRL" => *num * brl_to_inr_rate,
		"SAR" => *num * sar_to_inr_rate,
		"ILS" => *num * ils_to_inr_rate,
		"DKK" => *num * dkk_to_inr_rate,
		"PLN" => *num * pln_to_inr_rate,
		"MXN" => *num * mxn_to_inr_rate,
        "INR" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_krw(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_krw_rate = 0.00947;
	let eur_to_krw_rate = 0.01107;
	let jpy_to_krw_rate = 1.79059;
	let usd_to_krw_rate = 0.01208;
	let cad_to_krw_rate = 0.0163;
	let aud_to_krw_rate = 0.01831;
	let cny_to_krw_rate = 0.08694;
	let chf_to_krw_rate = 0.01063;
	let sek_to_krw_rate = 0.124;
	let inr_to_krw_rate = 0.0623;
	let nok_to_krw_rate = 0.00787;
	let nzd_to_krw_rate = 0.00122;
	let rub_to_krw_rate = 0.06878;
	let brl_to_krw_rate = 0.00375;
	let sar_to_krw_rate = 0.00284;
	let ils_to_krw_rate = 0.00271;
	let dkk_to_krw_rate = 0.00516;
	let pln_to_krw_rate = 0.00298;
	let mxn_to_krw_rate = 0.01276;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_krw_rate,
        "EUR" => *num * eur_to_krw_rate,
        "GBP" => *num * gbp_to_krw_rate,
        "JPY" => *num * jpy_to_krw_rate,
        "CAD" => *num * cad_to_krw_rate,
        "CNY" => *num * cny_to_krw_rate,
        "AUD" => *num * aud_to_krw_rate,
        "CHF" => *num * chf_to_krw_rate,
        "SEK" => *num * sek_to_krw_rate,
        "INR" => *num * inr_to_krw_rate,
		"NOK" => *num * nok_to_krw_rate,
		"NZD" => *num * nzd_to_krw_rate,
		"RUB" => *num * rub_to_krw_rate,
		"BRL" => *num * brl_to_krw_rate,
		"SAR" => *num * sar_to_krw_rate,
		"ILS" => *num * ils_to_krw_rate,
		"DKK" => *num * dkk_to_krw_rate,
		"PLN" => *num * pln_to_krw_rate,
		"MXN" => *num * mxn_to_krw_rate,
        "KRW" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_nok(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_nok_rate = 0.00947;
	let eur_to_nok_rate = 0.01107;
	let jpy_to_nok_rate = 1.79059;
	let usd_to_nok_rate = 0.01208;
	let cad_to_nok_rate = 0.0163;
	let aud_to_nok_rate = 0.01831;
	let cny_to_nok_rate = 0.08694;
	let chf_to_nok_rate = 0.01063;
	let sek_to_nok_rate = 0.124;
	let krw_to_nok_rate = 126.923;
	let inr_to_nok_rate = 7.91578;
	let nzd_to_nok_rate = 0.15533;
	let rub_to_nok_rate = 8.72469;
	let brl_to_nok_rate = 0.47622;
	let sar_to_nok_rate = 0.36022;
	let ils_to_nok_rate = 0.34313;
	let dkk_to_nok_rate = 0.6547;
	let pln_to_nok_rate = 0.37781;
	let mxn_to_nok_rate = 1.61873;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_nok_rate,
        "EUR" => *num * eur_to_nok_rate,
        "GBP" => *num * gbp_to_nok_rate,
        "JPY" => *num * jpy_to_nok_rate,
        "CAD" => *num * cad_to_nok_rate,
        "CNY" => *num * cny_to_nok_rate,
        "AUD" => *num * aud_to_nok_rate,
        "CHF" => *num * chf_to_nok_rate,
        "SEK" => *num * sek_to_nok_rate,
        "KRW" => *num * krw_to_nok_rate,
		"INR" => *num * inr_to_nok_rate,
		"NZD" => *num * nzd_to_nok_rate,
		"RUB" => *num * rub_to_nok_rate,
		"BRL" => *num * brl_to_nok_rate,
		"SAR" => *num * sar_to_nok_rate,
		"ILS" => *num * ils_to_nok_rate,
		"DKK" => *num * dkk_to_nok_rate,
		"PLN" => *num * pln_to_nok_rate,
		"MXN" => *num * mxn_to_nok_rate,
        "NOK" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_nzd(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_nzd_rate = 0.00947;
	let eur_to_nzd_rate = 0.01107;
	let jpy_to_nzd_rate = 1.79059;
	let usd_to_nzd_rate = 0.01208;
	let cad_to_nzd_rate = 0.0163;
	let aud_to_nzd_rate = 0.01831;
	let cny_to_nzd_rate = 0.08694;
	let chf_to_nzd_rate = 0.01063;
	let sek_to_nzd_rate = 0.124;
	let krw_to_nzd_rate = 816.913;
	let nok_to_nzd_rate = 6.4337;
	let inr_to_nzd_rate = 50.9484;
	let rub_to_nzd_rate = 56.0867;
	let brl_to_nzd_rate = 3.06139;
	let sar_to_nzd_rate = 2.31566;
	let ils_to_nzd_rate = 2.20578;
	let dkk_to_nzd_rate = 4.20873;
	let pln_to_nzd_rate = 2.42876;
	let mxn_to_nzd_rate = 10.406;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_nzd_rate,
        "EUR" => *num * eur_to_nzd_rate,
        "GBP" => *num * gbp_to_nzd_rate,
        "JPY" => *num * jpy_to_nzd_rate,
        "CAD" => *num * cad_to_nzd_rate,
        "CNY" => *num * cny_to_nzd_rate,
        "AUD" => *num * aud_to_nzd_rate,
        "CHF" => *num * chf_to_nzd_rate,
        "SEK" => *num * sek_to_nzd_rate,
        "INR" => *num * inr_to_nzd_rate,
        "KRW" => *num * krw_to_nzd_rate,
        "NOK" => *num * nok_to_nzd_rate,
        "RUB" => *num * rub_to_nzd_rate,
        "BRL" => *num * brl_to_nzd_rate,
        "SAR" => *num * sar_to_nzd_rate,
		"ILS" => *num * ils_to_nzd_rate,
		"DKK" => *num * dkk_to_nzd_rate,
		"PLN" => *num * pln_to_nzd_rate,
		"MXN" => *num * mxn_to_nzd_rate,
        "NZD" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_rub(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_rub_rate = 0.00858;
	let eur_to_rub_rate = 0.01006;
	let jpy_to_rub_rate = 1.62277;
	let usd_to_rub_rate = 0.01101;
	let cad_to_rub_rate = 0.01482;
	let aud_to_rub_rate = 0.0166;
	let cny_to_rub_rate = 0.07915;
	let chf_to_rub_rate = 0.00965;
	let sek_to_rub_rate = 0.11246;
	let krw_to_rub_rate = 14.5132;
	let nok_to_rub_rate = 0.11451;
	let inr_to_rub_rate = 0.91091;
	let nzd_to_rub_rate = 0.01782;
	let brl_to_rub_rate = 0.05456;
	let sar_to_rub_rate = 0.04127;
	let ils_to_rub_rate = 0.03931;
	let dkk_to_rub_rate = 0.075;
	let pln_to_rub_rate = 0.04328;
	let mxn_to_rub_rate = 0.18544;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_rub_rate,
        "EUR" => *num * eur_to_rub_rate,
        "GBP" => *num * gbp_to_rub_rate,
        "JPY" => *num * jpy_to_rub_rate,
        "CAD" => *num * cad_to_rub_rate,
        "CNY" => *num * cny_to_rub_rate,
        "AUD" => *num * aud_to_rub_rate,
        "CHF" => *num * chf_to_rub_rate,
        "SEK" => *num * sek_to_rub_rate,
        "INR" => *num * inr_to_rub_rate,
        "KRW" => *num * krw_to_rub_rate,
        "NOK" => *num * nok_to_rub_rate,
        "NZD" => *num * nzd_to_rub_rate,
        "BRL" => *num * brl_to_rub_rate,
        "SAR" => *num * sar_to_rub_rate,
		"ILS" => *num * ils_to_rub_rate,
		"DKK" => *num * dkk_to_rub_rate,
		"PLN" => *num * pln_to_rub_rate,
		"MXN" => *num * mxn_to_rub_rate,
        "RUB" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_brl(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_brl_rate = 0.15712;
	let eur_to_brl_rate = 0.1843;
	let jpy_to_brl_rate = 29.7238;
	let usd_to_brl_rate = 0.20168;
	let cad_to_brl_rate = 0.27143;
	let aud_to_brl_rate = 0.30412;
	let cny_to_brl_rate = 1.44972;
	let chf_to_brl_rate = 0.17684;
	let sek_to_brl_rate = 2.05999;
	let krw_to_brl_rate = 265.833;
	let nok_to_brl_rate = 2.09746;
	let inr_to_brl_rate = 16.6849;
	let nzd_to_brl_rate = 0.32634;
	let rub_to_brl_rate = 18.3077;
	let sar_to_brl_rate = 0.75587;
	let ils_to_brl_rate = 0.72001;
	let dkk_to_brl_rate = 1.37381;
	let pln_to_brl_rate = 0.79279;
	let mxn_to_brl_rate = 3.3967;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_brl_rate,
        "EUR" => *num * eur_to_brl_rate,
        "GBP" => *num * gbp_to_brl_rate,
        "JPY" => *num * jpy_to_brl_rate,
        "CAD" => *num * cad_to_brl_rate,
        "CNY" => *num * cny_to_brl_rate,
        "AUD" => *num * aud_to_brl_rate,
        "CHF" => *num * chf_to_brl_rate,
        "SEK" => *num * sek_to_brl_rate,
        "INR" => *num * inr_to_brl_rate,
        "KRW" => *num * krw_to_brl_rate,
        "NOK" => *num * nok_to_brl_rate,
        "NZD" => *num * nzd_to_brl_rate,
        "RUB" => *num * rub_to_brl_rate,
        "SAR" => *num * sar_to_brl_rate,
		"ILS" => *num * ils_to_brl_rate,
		"DKK" => *num * dkk_to_brl_rate,
		"PLN" => *num * pln_to_brl_rate,
		"MXN" => *num * mxn_to_brl_rate,
        "BRL" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_sar(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_sar_rate = 0.20756;
	let eur_to_sar_rate = 0.24346;
	let jpy_to_sar_rate = 39.265;
	let usd_to_sar_rate = 0.26642;
	let cad_to_sar_rate = 0.35856;
	let aud_to_sar_rate = 0.40174;
	let cny_to_sar_rate = 1.91508;
	let chf_to_sar_rate = 0.23361;
	let sek_to_sar_rate = 2.72124;
	let krw_to_sar_rate = 351.165;
	let nok_to_sar_rate = 2.77075;
	let inr_to_sar_rate = 22.0408;
	let nzd_to_sar_rate = 0.43109;
	let rub_to_sar_rate = 24.1844;
	let brl_to_sar_rate = 1.32006;
	let ils_to_sar_rate = 0.95113;
	let dkk_to_sar_rate = 1.81479;
	let pln_to_sar_rate = 1.04727;
	let mxn_to_sar_rate = 4.48703;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_sar_rate,
        "EUR" => *num * eur_to_sar_rate,
        "GBP" => *num * gbp_to_sar_rate,
        "JPY" => *num * jpy_to_sar_rate,
        "CAD" => *num * cad_to_sar_rate,
        "CNY" => *num * cny_to_sar_rate,
        "AUD" => *num * aud_to_sar_rate,
        "CHF" => *num * chf_to_sar_rate,
        "SEK" => *num * sek_to_sar_rate,
        "INR" => *num * inr_to_sar_rate,
        "KRW" => *num * krw_to_sar_rate,
        "NOK" => *num * nok_to_sar_rate,
        "NZD" => *num * nzd_to_sar_rate,
        "RUB" => *num * rub_to_sar_rate,
		"ILS" => *num * ils_to_sar_rate,
		"BRL" => *num * brl_to_sar_rate,
		"DKK" => *num * dkk_to_sar_rate,
		"PLN" => *num * pln_to_sar_rate,
		"MXN" => *num * mxn_to_sar_rate,
        "SAR" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_ils(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_ils_rate = 0.21747;
	let eur_to_ils_rate = 0.25509;
	let jpy_to_ils_rate = 41.14;
	let usd_to_ils_rate = 0.27914;
	let cad_to_ils_rate = 0.37568;
	let aud_to_ils_rate = 0.42092;
	let cny_to_ils_rate = 2.00653;
	let chf_to_ils_rate = 0.24476;
	let sek_to_ils_rate = 2.85119;
	let krw_to_ils_rate = 367.933;
	let nok_to_ils_rate = 2.90305;
	let inr_to_ils_rate = 23.0932;
	let nzd_to_ils_rate = 0.45167;
	let rub_to_ils_rate = 25.3392;
	let brl_to_ils_rate = 1.38309;
	let sar_to_ils_rate = 1.04619;
	let dkk_to_ils_rate = 1.90145;
	let pln_to_ils_rate = 1.09728;
	let mxn_to_ils_rate = 4.70129;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_ils_rate,
        "EUR" => *num * eur_to_ils_rate,
        "GBP" => *num * gbp_to_ils_rate,
        "JPY" => *num * jpy_to_ils_rate,
        "CAD" => *num * cad_to_ils_rate,
        "CNY" => *num * cny_to_ils_rate,
        "AUD" => *num * aud_to_ils_rate,
        "CHF" => *num * chf_to_ils_rate,
        "SEK" => *num * sek_to_ils_rate,
        "INR" => *num * inr_to_ils_rate,
        "KRW" => *num * krw_to_ils_rate,
        "NOK" => *num * nok_to_ils_rate,
        "NZD" => *num * nzd_to_ils_rate,
        "RUB" => *num * rub_to_ils_rate,
        "SAR" => *num * sar_to_ils_rate,
		"BRL" => *num * brl_to_ils_rate,
		"DKK" => *num * dkk_to_ils_rate,
		"PLN" => *num * pln_to_ils_rate,
		"MXN" => *num * mxn_to_ils_rate,
        "ILS" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_dkk(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_dkk_rate = 0.11435;
	let eur_to_dkk_rate = 0.13414;
	let jpy_to_dkk_rate = 21.631;
	let usd_to_dkk_rate = 0.14677;
	let cad_to_dkk_rate = 0.19753;
	let aud_to_dkk_rate = 0.22132;
	let cny_to_dkk_rate = 1.05502;
	let chf_to_dkk_rate = 0.12869;
	let sek_to_dkk_rate = 1.49913;
	let krw_to_dkk_rate = 193.456;
	let nok_to_dkk_rate = 1.5264;
	let inr_to_dkk_rate = 12.1422;
	let nzd_to_dkk_rate = 0.23749;
	let rub_to_dkk_rate = 13.3232;
	let brl_to_dkk_rate = 0.72722;
	let sar_to_dkk_rate = 0.55008;
	let ils_to_dkk_rate = 0.52397;
	let pln_to_dkk_rate = 0.57694;
	let mxn_to_dkk_rate = 2.4719;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_dkk_rate,
        "EUR" => *num * eur_to_dkk_rate,
        "GBP" => *num * gbp_to_dkk_rate,
        "JPY" => *num * jpy_to_dkk_rate,
        "CAD" => *num * cad_to_dkk_rate,
        "CNY" => *num * cny_to_dkk_rate,
        "AUD" => *num * aud_to_dkk_rate,
        "CHF" => *num * chf_to_dkk_rate,
        "SEK" => *num * sek_to_dkk_rate,
        "INR" => *num * inr_to_dkk_rate,
        "KRW" => *num * krw_to_dkk_rate,
        "NOK" => *num * nok_to_dkk_rate,
        "NZD" => *num * nzd_to_dkk_rate,
        "RUB" => *num * rub_to_dkk_rate,
        "SAR" => *num * sar_to_dkk_rate,
		"ILS" => *num * ils_to_dkk_rate,
		"BRL" => *num * brl_to_dkk_rate,
		"PLN" => *num * pln_to_dkk_rate,
		"MXN" => *num * mxn_to_dkk_rate,
        "DKK" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_pln(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_pln_rate = 0.19807;
	let eur_to_pln_rate = 0.23236;
	let jpy_to_pln_rate = 37.4699;
	let usd_to_pln_rate = 0.25424;
	let cad_to_pln_rate = 0.34217;
	let aud_to_pln_rate = 0.38337;
	let cny_to_pln_rate = 1.82753;
	let chf_to_pln_rate = 0.22293;
	let sek_to_pln_rate = 2.59683;
	let krw_to_pln_rate = 335.11;
	let nok_to_pln_rate = 2.64407;
	let inr_to_pln_rate = 21.0331;
	let nzd_to_pln_rate = 0.41138;
	let rub_to_pln_rate = 23.0787;
	let brl_to_pln_rate = 1.25971;
	let sar_to_pln_rate = 0.95286;
	let ils_to_pln_rate = 0.90764;
	let dkk_to_pln_rate = 1.73182;
	let mxn_to_pln_rate = 4.28189;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_pln_rate,
        "EUR" => *num * eur_to_pln_rate,
        "GBP" => *num * gbp_to_pln_rate,
        "JPY" => *num * jpy_to_pln_rate,
        "CAD" => *num * cad_to_pln_rate,
        "CNY" => *num * cny_to_pln_rate,
        "AUD" => *num * aud_to_pln_rate,
        "CHF" => *num * chf_to_pln_rate,
        "SEK" => *num * sek_to_pln_rate,
        "INR" => *num * inr_to_pln_rate,
        "KRW" => *num * krw_to_pln_rate,
        "NOK" => *num * nok_to_pln_rate,
        "NZD" => *num * nzd_to_pln_rate,
        "RUB" => *num * rub_to_pln_rate,
        "SAR" => *num * sar_to_pln_rate,
		"ILS" => *num * ils_to_pln_rate,
		"BRL" => *num * brl_to_pln_rate,
		"DKK" => *num * dkk_to_pln_rate,
		"MXN" => *num * mxn_to_pln_rate,
        "PLN" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_mxn(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_mxn_rate = 0.04625;
	let eur_to_mxn_rate = 0.05424;
	let jpy_to_mxn_rate = 8.74841;
	let usd_to_mxn_rate = 0.05936;
	let cad_to_mxn_rate = 0.07989;
	let aud_to_mxn_rate = 0.08951;
	let cny_to_mxn_rate = 0.42669;
	let chf_to_mxn_rate = 0.05205;
	let sek_to_mxn_rate = 0.6063;
	let krw_to_mxn_rate = 78.241;
	let nok_to_mxn_rate = 0.61733;
	let inr_to_mxn_rate = 4.91077;
	let nzd_to_mxn_rate = 0.09605;
	let rub_to_mxn_rate = 5.38839;
	let brl_to_mxn_rate = 0.29412;
	let sar_to_mxn_rate = 0.22247;
	let ils_to_mxn_rate = 0.21191;
	let dkk_to_mxn_rate = 0.40434;
	let pln_to_mxn_rate = 0.23334;

	let amount: f32 = match to_currency {
        "USD" => *num * usd_to_mxn_rate,
        "EUR" => *num * eur_to_mxn_rate,
        "GBP" => *num * gbp_to_mxn_rate,
        "JPY" => *num * jpy_to_mxn_rate,
        "CAD" => *num * cad_to_mxn_rate,
        "CNY" => *num * cny_to_mxn_rate,
        "AUD" => *num * aud_to_mxn_rate,
        "CHF" => *num * chf_to_mxn_rate,
        "SEK" => *num * sek_to_mxn_rate,
        "INR" => *num * inr_to_mxn_rate,
        "KRW" => *num * krw_to_mxn_rate,
        "NOK" => *num * nok_to_mxn_rate,
        "NZD" => *num * nzd_to_mxn_rate,
        "RUB" => *num * rub_to_mxn_rate,
        "SAR" => *num * sar_to_mxn_rate,
		"ILS" => *num * ils_to_mxn_rate,
		"BRL" => *num * brl_to_mxn_rate,
		"DKK" => *num * dkk_to_mxn_rate,
		"PLN" => *num * pln_to_mxn_rate,
        "MXN" => *num,
        _ => 0.0,
    };

    amount
}

pub fn dave_currency_conv(
	amount: f32,
	from_currency: &str,
	to_currency: &str,
) -> io::Result<()> {
	match from_currency {
		"USD" => {
			match Currency::from_code("USD") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"GBP" => {
			match Currency::from_code("GBP") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"EUR" => {
			match Currency::from_code("EUR") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					print!("##==> Nations that use the {}: ", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"JPY" => {
			match Currency::from_code("JPY") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"CAD" => {
			match Currency::from_code("CAD") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"CNY" => {
			match Currency::from_code("CNY") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"AUD" => {
			match Currency::from_code("AUD") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"CHF" => {
			match Currency::from_code("CHF") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"SEK" => {
			match Currency::from_code("SEK") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"INR" => {
			match Currency::from_code("INR") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"KRW" => {
			match Currency::from_code("KRW") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"NOK" => {
			match Currency::from_code("NOK") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"NZD" => {
			match Currency::from_code("NZD") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"RUB" => {
			match Currency::from_code("RUB") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"BRL" => {
			match Currency::from_code("BRL") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"SAR" => {
			match Currency::from_code("SAR") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"ILS" => {
			match Currency::from_code("ILS") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"DKK" => {
			match Currency::from_code("DKK") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"PLN" => {
			match Currency::from_code("PLN") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"MXN" => {
			match Currency::from_code("MXN") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		_ => {
			eprintln!("##==>>>> ERROR: Unknown Currency Type");
		},
	}

	let result = match_from_currency(from_currency, to_currency, &amount);
	let exchange_result = ExchangeResult {
        original_currency: String::from(&from_currency.trim().to_uppercase()),
        original_amount:  amount,
        final_currency: String::from(&to_currency.trim().to_uppercase()),
        final_amount: result
    };

	println!(
		"{} {} was converted to {} {}",
		exchange_result.original_currency,
		exchange_result.original_amount,
		exchange_result.final_currency,
		exchange_result.final_amount,
	);
	
	Ok(())
}
