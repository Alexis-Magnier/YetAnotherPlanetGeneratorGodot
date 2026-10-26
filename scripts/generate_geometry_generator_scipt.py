
# python3 generate_geometry_generator_scipt.py | xclip -selection clipboard

properties = {
	"cells": {
		"seed": ("int", "10000"),
		"jitter": ("float", "0.5"),
		"cell_count": ("int", "0"),
	},
	"plates": {
		"seed": ("int", "0"),
		"plate_count": ("int", "150"),
		"priority_dev": ("float", "0.5"),
		"priority_mean": ("float", "0.5"),
		"directionnal_preference_dev": ("float", "0.5"),
		"directionnal_preference_mean": ("float", "0.5"),
		"velocity_mean": ("float", "15.0"),
		"velocity_dev": ("float", "1.5"),
		"materials": ("Array[YAPG_PlateMaterial]", "[]"),
	},
	"collision": {
		"dt": ("float", "0.1"),
		"boundary_threshold": ("float", "5.0"),
		"weight_stress_factor": ("float", "1.0"),
		"self_subduction_height": ("float", "-0.1"),
		"other_subduction_height": ("float", "0.5"),
	},
	"properties": {
		"density_noise_seed": ("int", "0"),
		"height_noise_seed": ("int", "0"),
		"octaves": ("int", "3"),
		"frequency": ("float", "8.0"),
		"lacunarity": ("float", "2.0"),
		"persistence": ("float", "0.5"),
	},
	"temperature": {
		"divergent_temp": ("float", "1.0"),
		"convergent_temp": ("float", "0.4"),
		"transform_temp": ("float", "0.1"),
		"cooldown_factor": ("float", "0.1"),
		"diffuse_steps": ("int", "3"),
		"diffuse_weight": ("float", "0.1"),
		"diffuse_coef": ("float", "0.25"),
		"noise_seed": ("int", "6222"),
		"noise_frequency": ("float", "2.0"),
		"noise_amplitude": ("float", "0.1"),
		"noise_lacunarity": ("float", "2.5"),
		"noise_persistence": ("float", "0.5"),
	},
	"strength": {
		"noise_seed": ("int", "0"),
		"noise_octaves": ("int", "6"),
		"noise_frequency": ("float", "1.0"),
		"noise_lacunarity": ("float", "5.0"),
		"noise_persistence": ("float", "0.5"),
		"noise_amplitude": ("float", "1.0"),
		"noise_mean": ("float", "0.0"),
		"noise_weight": ("float", "1.0"),
		"thermal_weight": ("float", "0.1"),
	},
	"stress": {
		"transmission_threshold": ("float", "0.1"),
		"base_strength": ("float", "0.1"),
		"strength_mult": ("float", "1.0"),
		"base_stress_mult": ("float", "1.0"),
		"reference_distance": ("float", "1.0"),
		"diffuse_weight": ("float", "0.2"),
		"diffuse_steps": ("int", "5"),
		"noise_seed": ("int", "0"),
		"noise_octaves": ("int", "6"),
		"noise_frequency": ("float", "1.0"),
		"noise_amplitude": ("float", "0.5"),
		"noise_lacunarity": ("float", "1.0"),
		"noise_persistence": ("float", "5.0"),
		"noise_strength_weight": ("float", "2.0"),
		"noise_max": ("float", "0.1"),
	},
	"isostasy": {
		"phase_iterations": ("int", "3.3"),
		"mantle_density": ("float", "48"),
		"fold_wavelength": ("float", "1.0"),
		"fold_gain": ("float", "1.2"),
		"fault_spacing": ("float", "45.0"),
		"fault_gain": ("float", "1.0"),
		"yeild_stress": ("float", "0.25"),
		"reference_thickness": ("float", "0.35"),
		"smoothing_iterations": ("int", "2"),
		"seed": ("int", "0x9E37_79B9"),
	}
}

export_snippet = """
@export var {group}_{property}: {type} = {default}:
	set(val):
		{group}_{property} = val
		if interface != null:
			interface.set_{group}_{property}(val)
	get():
		return {group}_{property}"""

default_val_snippet = "interface.set_{group}_{property}({group}_{property})"

def for_each_property(fn: callable):
	for group, props in properties.items():
		for prop in props.items():
			fn(
				group=group,
				property=prop[0],
				type=prop[1][0],
				default=prop[1][1]
			)


def main():
	for group, props in properties.items():
		print(f"\n\n@export_group(\"{group}\", \"{group}_\")", end="")
		for prop in props.items():
			print(
				export_snippet.format(
					group=group,
					property=prop[0],
					type=prop[1][0],
					default=prop[1][1]
				),
				end=""
			)
	

	print("\n\nfunc set_defaults():")
	for_each_property(lambda group, property, type, default:
		print("\t" + default_val_snippet.format(group=group, property=property, type=type))
	)

if __name__ == "__main__":
	main()

