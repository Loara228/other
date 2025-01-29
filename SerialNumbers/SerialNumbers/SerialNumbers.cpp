#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <thread>
#include <windows.h>
#include <vector>

void Send(const std::string& text) {
    for (char c : text) {
        INPUT input;
        input.type = INPUT_KEYBOARD;
        input.ki.wScan = 0;
        input.ki.time = 0;
        input.ki.dwExtraInfo = 0;

        WORD vk = VkKeyScan(c);

        input.ki.wVk = vk;
        input.ki.dwFlags = 0;
        SendInput(1, &input, sizeof(INPUT));

        input.ki.dwFlags = KEYEVENTF_KEYUP;
        SendInput(1, &input, sizeof(INPUT));
    }

    INPUT enterInput;
    enterInput.type = INPUT_KEYBOARD;
    enterInput.ki.wScan = 0;
    enterInput.ki.time = 0;
    enterInput.ki.dwExtraInfo = 0;

    enterInput.ki.wVk = VK_RETURN;
    enterInput.ki.dwFlags = 0;
    SendInput(1, &enterInput, sizeof(INPUT));
    std::this_thread::sleep_for(std::chrono::milliseconds(15));

    enterInput.ki.dwFlags = KEYEVENTF_KEYUP;
    SendInput(1, &enterInput, sizeof(INPUT));
    std::this_thread::sleep_for(std::chrono::milliseconds(15));
}

int main()
{
    std::ifstream inputFile("input.txt");
    if (!inputFile.is_open()) {
        std::cerr << "+ \"input.txt\" not found" << std::endl;
        return 1;
    }

    std::vector<std::string> lines;
    std::string line;
    while (std::getline(inputFile, line)) {
        lines.push_back(line);
        std::cout << line << std::endl;
    }

    inputFile.close();

    for (size_t i = 3; i > 0; i--)
    {
        std::cout << std::to_string(i) + "..." << std::endl;
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    for (const std::string& l : lines) {
        Send(l);
    }
}