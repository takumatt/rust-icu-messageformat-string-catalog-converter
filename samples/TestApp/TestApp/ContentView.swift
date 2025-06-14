//
//  ContentView.swift
//  TestApp
//
//  Created by Takuma Matsushita on 2025/05/29.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
          Text(String(format: String(localized: "hello", table: "simple_argument"), "Alice", "Bob"))
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
