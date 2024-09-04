/*
 * SPDX-FileCopyrightText: 2024 Eduardo Javier Alvarado Aarón <eduardo.javier.alvarado.aaron@gmail.com>
 *
 * SPDX-License-Identifier: Unlicense
 */

fn __() { // XML
	let _ = br##"<tag a={var} b="{var}">text \" {var}</tag>"##;
	let _ = br##"<tag a={var} b="{var}">text \" {var}</tag>"##; // not highlighted
	
	// XML label
	let _ = br##"
		<tag a={var} b="{var}">text \" {var}</tag>
	"##;
	let _ = br##"<tag a={var} b="{var}">text \" {var}</tag>"##; // not highlighted
	
	let _ = /* XML label */ br##"
		<tag a={var} b="{var}">text \" {var}</tag>
	"##;
	let _ = br##""<tag a={var} b="{var}">text \" {var}</tag>"##; // not highlighted
}
