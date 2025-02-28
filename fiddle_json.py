import json


def change_pixels_to_fraction(input_path, output_path, convert_to_fractions):
    width = 1080
    height = 1920
    fractioned_mirrors = []

    with open(input_path, "r") as read:
        json_data = json.load(read)

        mirrors = json_data["mirrors"]

        for mirror in mirrors:
            start_pos = mirror["start_pos"]
            end_pos = mirror["end_pos"]
            if convert_to_fractions:
                new_start_pos = [float(start_pos[0] / width), float(start_pos[1] / height)]
                new_end_pos = [float(end_pos[0] / width), float(end_pos[1] / height)]
            else:
                new_start_pos = [float(start_pos[0]), float(start_pos[1])]
                new_end_pos = [float(end_pos[0]), float(end_pos[1])]
            new_mirror = {
                "start_pos": new_start_pos,
                "end_pos": new_end_pos,
                "absorption_factor": mirror["absorption_factor"]
            }
            fractioned_mirrors.append(new_mirror)

    with open(output_path, "w+") as write:
        json_data = {
            "mirrors": fractioned_mirrors
        }
        json.dump(json_data, write)


change_pixels_to_fraction("assets/mirrors.json", "assets/mirrors.json", False)
